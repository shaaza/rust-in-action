# Chapter 07: ActionKV

`actionkv` is a small file-backed key-value store CLI from chapter 07. It stores
string keys and values in an append-only data file and supports basic `get`,
`insert`, `update`, and `delete` operations.

## Incremental Additions

- **Store trait**: split the key-value store behind a clear storage interface
  that the CLI can use.
- **Separate library crate**: moved the store into `libactionkv` and started
  with an in-memory implementation.
- **File opening**: create or reuse a store file path from the CLI.
- **File-backed persistence**: keep inserted values available across later CLI
  invocations.
- **Bitcask-style data format**: append records to disk and keep keydir entries
  for fast lookups.
- **Parity-bit checking**: reject corrupted records while reading.
- **Criterion benchmarks**: measure open, get, and insert performance.
- **Persisted on-disk index**: write a sidecar (`<store-file>.idx`) to reduce
  startup cost when reopening an existing store.

## Usage

Run commands from the `code` directory:

```sh
cd code
cargo run -- store.db insert my-key my-value
cargo run -- store.db get my-key
cargo run -- store.db update my-key new-value
cargo run -- store.db delete my-key
```

The first argument is the store file path. If it does not exist, the app creates
it. The CLI also creates a sidecar index file next to it, such as
`store.db.idx`.

Example output:

```text
insert my-key=my-value into "store.db"
my-key=my-value
update my-key=new-value in "store.db"
delete my-key from "store.db"
```

Run the test suite:

```sh
cd code
cargo test
```

Run benchmarks:

```sh
cd code/libactionkv
cargo bench
```

Benchmark notes and recent results are in `code/libactionkv/BENCHMARKS.md`.
