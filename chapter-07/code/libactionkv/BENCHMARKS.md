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
- `Store::get` for indexed lookup followed by record validation and value read.
- `Store::insert` for appending a new record and updating the in-memory index.

The main decision point is `KVStore::open`. Today it reads the entire data file
and rebuilds the in-memory index by decoding every record. If that benchmark
scales poorly enough for expected store sizes, it is evidence that loading a
persisted index from disk may be worth implementing.

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
| `open_rebuilds_index/100` | `[51.762 us 52.552 us 53.855 us]` | `[1.8569 Melem/s 1.9029 Melem/s 1.9319 Melem/s]` |
| `open_rebuilds_index/1000` | `[248.79 us 252.54 us 259.90 us]` | `[3.8476 Melem/s 3.9597 Melem/s 4.0194 Melem/s]` |
| `open_rebuilds_index/10000` | `[2.2092 ms 2.2742 ms 2.3901 ms]` | `[4.1839 Melem/s 4.3972 Melem/s 4.5265 Melem/s]` |
| `get_existing_key` | `[1.5439 us 1.5845 us 1.6418 us]` | n/a |
| `insert_new_key` | `[4.2518 us 4.4938 us 4.8077 us]` | n/a |

Raw output:

```text
Finished `bench` profile [optimized] target(s) in 0.23s
Running unittests src/lib.rs (target/release/deps/libactionkv-6ec537ccb12f354c)

running 11 tests
test record::tests::decode_reads_delete_record_into_tombstone ... ignored
test record::tests::decode_reads_record_at_offset_and_returns_next_offset ... ignored
test record::tests::decode_reads_upsert_record_into_keydir_entry ... ignored
test record::tests::decode_rejects_checksum_mismatch ... ignored
test record::tests::decode_rejects_delete_record_with_value ... ignored
test record::tests::decode_rejects_incomplete_body ... ignored
test record::tests::decode_rejects_incomplete_header ... ignored
test record::tests::decode_rejects_unknown_record_kind ... ignored
test record::tests::decode_returns_none_at_end_of_file ... ignored
test record::tests::encode_writes_delete_record_as_kind_timestamp_key_size_zero_value_size_key ... ignored
test record::tests::encode_writes_upsert_record_as_kind_timestamp_sizes_key_value ... ignored

test result: ok. 0 passed; 0 failed; 11 ignored; 0 measured; 0 filtered out; finished in 0.00s

Running benches/store.rs (target/release/deps/store-97e9563a30bb1883)
Gnuplot not found, using plotters backend
Benchmarking open_rebuilds_index/100
Benchmarking open_rebuilds_index/100: Warming up for 3.0000 s
Benchmarking open_rebuilds_index/100: Collecting 20 samples in estimated 5.0075 s (96k iterations)
Benchmarking open_rebuilds_index/100: Analyzing
open_rebuilds_index/100 time:   [51.762 us 52.552 us 53.855 us]
                        thrpt:  [1.8569 Melem/s 1.9029 Melem/s 1.9319 Melem/s]
Found 3 outliers among 20 measurements (15.00%)
  1 (5.00%) high mild
  2 (10.00%) high severe
Benchmarking open_rebuilds_index/1000
Benchmarking open_rebuilds_index/1000: Warming up for 3.0000 s
Benchmarking open_rebuilds_index/1000: Collecting 20 samples in estimated 5.0157 s (20k iterations)
Benchmarking open_rebuilds_index/1000: Analyzing
open_rebuilds_index/1000
                        time:   [248.79 us 252.54 us 259.90 us]
                        thrpt:  [3.8476 Melem/s 3.9597 Melem/s 4.0194 Melem/s]
Found 2 outliers among 20 measurements (10.00%)
  1 (5.00%) high mild
  1 (5.00%) high severe
Benchmarking open_rebuilds_index/10000
Benchmarking open_rebuilds_index/10000: Warming up for 3.0000 s
Benchmarking open_rebuilds_index/10000: Collecting 20 samples in estimated 5.3318 s (2310 iterations)
Benchmarking open_rebuilds_index/10000: Analyzing
open_rebuilds_index/10000
                        time:   [2.2092 ms 2.2742 ms 2.3901 ms]
                        thrpt:  [4.1839 Melem/s 4.3972 Melem/s 4.5265 Melem/s]
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) high severe

Benchmarking get_existing_key
Benchmarking get_existing_key: Warming up for 3.0000 s
Benchmarking get_existing_key: Collecting 20 samples in estimated 5.0000 s (3.1M iterations)
Benchmarking get_existing_key: Analyzing
get_existing_key        time:   [1.5439 us 1.5845 us 1.6418 us]
Found 2 outliers among 20 measurements (10.00%)
  2 (10.00%) high severe

Benchmarking insert_new_key
Benchmarking insert_new_key: Warming up for 3.0000 s
Benchmarking insert_new_key: Collecting 20 samples in estimated 5.0006 s (1.1M iterations)
Benchmarking insert_new_key: Analyzing
insert_new_key          time:   [4.2518 us 4.4938 us 4.8077 us]
Found 7 outliers among 20 measurements (35.00%)
  4 (20.00%) low mild
  1 (5.00%) high mild
  2 (10.00%) high severe
```
