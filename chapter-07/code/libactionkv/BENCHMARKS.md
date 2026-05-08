# Benchmarks

This crate uses Criterion for Cargo benchmarks. Criterion gives us stable Rust
benchmarks, repeated measurements, outlier handling, and local HTML reports
without relying on the nightly-only built-in benchmark harness.

Run the benchmarks from this directory:

```sh
cargo bench
```

Compile the benchmark target without running it:

```sh
cargo bench --no-run
```

Criterion writes reports under `target/criterion/`.

## Benchmark Stack

- Benchmark harness: `criterion`
- Benchmark target: `benches/store.rs`
- Cargo configuration: `[[bench]] name = "store", harness = false`
- Temporary storage: `tempfile`
- Measurement sample size: `20`
- Black-boxing: `criterion::black_box` prevents the optimizer from removing
  the operation being measured.

## Workload Type

The workloads use local, file-backed stores created in temporary directories.
Keys and values are deterministic so benchmark runs are repeatable:

- Keys use the shape `key-00000000`.
- Values use the shape `value-00000000-` followed by 128 `x` bytes.
- Store sizes for open benchmarks are 100, 1,000, and 10,000 records.

Setup work is kept outside the timed loop where possible. For example,
`open_rebuilds_index` creates store files before timing begins, then measures
only reopening the existing file.

## What Is Exercised

The benchmarks exercise the public `KVStore` and `Store` APIs rather than
private helpers. This keeps the numbers aligned with the cost callers actually
pay:

- `KVStore::open` for loading a data file and rebuilding the in-memory keydir.
- `KVStore::open_with_persisted_index` for loading the keydir from an adjacent
  `.idx` snapshot when it is current.
- `Store::get` for indexed lookup followed by record validation and value read.
- `Store::insert` for appending a new record and updating the in-memory index.

The main decision point is `KVStore::open`. Today it reads the entire data file
and rebuilds the in-memory index by decoding every record. If that benchmark
scales poorly enough for expected store sizes, it is evidence that loading a
persisted index from disk may be worth implementing.

## Persisted Index Options

The implemented option is an adjacent binary snapshot file named by appending
`.idx` to the data file path, for example `store.db.idx`.
`KVStore::open_with_persisted_index` loads this file when its recorded data-file
length still matches the current data file. If it is missing or stale, the store
falls back to the original full scan and writes a fresh snapshot.

The snapshot stores only the in-memory keydir entries: key, record offset,
record size, value offset, value size, and timestamp. The data file remains the
source of truth, and `KVStore::open` is still available as the full-scan
correctness baseline.

Other viable designs:

- Snapshot on close or explicit checkpoint: faster writes, but a crash can leave
  the next open doing a full rebuild.
- Append-only index log: much cheaper per write, but it needs compaction and
  recovery logic similar to the main data file.
- Memory-mapped index file: useful for very large indexes, but more platform
  and lifetime complexity than this chapter-sized store needs.
- Embedded index records in the data file: keeps one file, but startup still has
  to find the latest valid index checkpoint before replaying newer data records.

## Benchmarks

### `open_rebuilds_index`

Builds temporary store files with 100, 1,000, and 10,000 inserted records. The
timed operation reopens each existing file with `KVStore::open`.

This measures the startup cost of rebuilding the keydir from the append-only
data file. It includes reading the whole file, decoding records, validating
checksums, allocating key strings, and inserting keydir entries into the
`HashMap`.

Criterion records throughput as records processed per measurement.

### `get_existing_key`

Builds a 10,000-record store, opens it once, then repeatedly reads existing
keys from different positions in the keyspace.

This measures steady-state lookup cost after the in-memory index already
exists. It exercises the `HashMap` lookup, targeted file read, record decode
and checksum validation, and UTF-8 value construction.

This benchmark is a baseline for normal read performance. A persisted on-disk
index should improve open time without making this path worse.

### `insert_new_key`

Creates an empty temporary store, opens it once, then repeatedly inserts new
unique keys.

This measures append-write cost plus in-memory index maintenance. It exercises
record encoding, checksum generation, seek-to-end append, flush, and insertion
of the new keydir entry into the `HashMap`.

This benchmark is a baseline for write performance. If an on-disk index is
added later, this workload can show the extra write cost of maintaining it.

## Latest Results

Collected on May 8, 2026 with:

```sh
cargo bench
```

Criterion reported that Gnuplot was not installed and used the plotters backend
for report generation.

| Benchmark | Time | Throughput |
| --- | ---: | ---: |
| `open_rebuilds_index/100` | `[51.344 us 52.749 us 55.108 us]` | `[1.8146 Melem/s 1.8958 Melem/s 1.9477 Melem/s]` |
| `open_persisted_index/100` | `[49.231 us 50.583 us 52.678 us]` | `[1.8983 Melem/s 1.9770 Melem/s 2.0312 Melem/s]` |
| `open_rebuilds_index/1000` | `[247.11 us 254.47 us 263.40 us]` | `[3.7966 Melem/s 3.9297 Melem/s 4.0468 Melem/s]` |
| `open_persisted_index/1000` | `[176.02 us 183.52 us 193.55 us]` | `[5.1666 Melem/s 5.4490 Melem/s 5.6812 Melem/s]` |
| `open_rebuilds_index/10000` | `[2.2316 ms 2.3327 ms 2.4265 ms]` | `[4.1212 Melem/s 4.2869 Melem/s 4.4812 Melem/s]` |
| `open_persisted_index/10000` | `[1.4557 ms 1.5114 ms 1.5931 ms]` | `[6.2770 Melem/s 6.6164 Melem/s 6.8695 Melem/s]` |
| `get_existing_key` | `[1.5607 us 1.6122 us 1.6835 us]` | n/a |
| `insert_new_key` | `[4.4311 us 4.7814 us 5.1936 us]` | n/a |
| `insert_new_key_with_persisted_index` | `[944.08 us 1.0149 ms 1.0669 ms]` | n/a |

The persisted snapshot improved median open time by about 4% at 100 records,
28% at 1,000 records, and 35% at 10,000 records in this run. The eager snapshot
write strategy is costly for inserts because each mutation rewrites the whole
index file; median insert time rose from 4.7814 us to 1.0149 ms. For a
write-heavy store, prefer an append-only index log or checkpoint-on-close
design.
