use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use libactionkv::{KVStore, Store};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

const STORE_SIZES: &[usize] = &[100, 1_000, 10_000];
const VALUE_SIZE: usize = 128;

struct StoreFixture {
    _dir: TempDir,
    filepath: PathBuf,
    record_count: usize,
}

fn build_store(record_count: usize) -> StoreFixture {
    let dir = tempfile::tempdir().expect("create benchmark tempdir");
    let filepath = dir.path().join("store.db");

    {
        let mut store = KVStore::open(filepath.clone()).expect("open benchmark store");
        for n in 0..record_count {
            let key = key(n);
            let value = value(n);
            store.insert(&key, &value).expect("insert benchmark record");
        }
    }

    StoreFixture {
        _dir: dir,
        filepath,
        record_count,
    }
}

fn open_store(filepath: &Path) {
    let store = KVStore::open(filepath.to_path_buf()).expect("open benchmark store");
    black_box(store);
}

fn key(n: usize) -> String {
    format!("key-{n:08}")
}

fn value(n: usize) -> String {
    format!("value-{n:08}-{}", "x".repeat(VALUE_SIZE))
}

fn bench_open_rebuilds_index(c: &mut Criterion) {
    let fixtures: Vec<_> = STORE_SIZES.iter().copied().map(build_store).collect();

    let mut group = c.benchmark_group("open_rebuilds_index");
    for fixture in &fixtures {
        group.throughput(Throughput::Elements(fixture.record_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(fixture.record_count),
            &fixture.filepath,
            |b, filepath| {
                b.iter(|| open_store(filepath));
            },
        );
    }
    group.finish();
}

fn bench_get_existing_key(c: &mut Criterion) {
    let fixture = build_store(10_000);
    let keys: Vec<_> = [0, 999, 4_999, 9_999].into_iter().map(key).collect();
    let mut store = KVStore::open(fixture.filepath).expect("open benchmark store");
    let mut next_key = 0;

    c.bench_function("get_existing_key", |b| {
        b.iter(|| {
            let key = &keys[next_key % keys.len()];
            next_key += 1;
            black_box(store.get(black_box(key)).expect("get benchmark record"));
        });
    });
}

fn bench_insert_new_key(c: &mut Criterion) {
    let dir = tempfile::tempdir().expect("create benchmark tempdir");
    let filepath = dir.path().join("store.db");
    let mut store = KVStore::open(filepath).expect("open benchmark store");
    let mut next_key = 0;

    c.bench_function("insert_new_key", |b| {
        b.iter(|| {
            let key = key(next_key);
            let value = value(next_key);
            next_key += 1;
            black_box(
                store
                    .insert(black_box(&key), black_box(&value))
                    .expect("insert benchmark record"),
            );
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = bench_open_rebuilds_index, bench_get_existing_key, bench_insert_new_key
);
criterion_main!(benches);
