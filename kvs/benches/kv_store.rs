use criterion::BatchSize::{LargeInput, NumIterations, SmallInput};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use kvs::{KvStoreV2, KvsEngine, MemStore, SledStore};
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng, SeedableRng,
};
use std::fs::{create_dir, remove_dir_all};
use tempfile;

/// Generate `n` strings of length between 1 and `m`, using a specified `seed`.
fn generate_strings(n: usize, m: usize, seed: u64) -> Vec<String> {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let mut strings: Vec<String> = Vec::new();
    for _ in 0..n {
        let len = rng.gen_range(1..m);
        let string: String = Alphanumeric.sample_string(&mut rng, len);
        strings.push(string.clone());
    }
    strings
}

pub fn bench_write_read(c: &mut Criterion) {
    let temp_dir_1 = tempfile::tempdir().unwrap();
    let temp_dir_2 = tempfile::tempdir().unwrap();
    let keys: Vec<String> = generate_strings(10, 100_000, 0);
    let values: Vec<String> = generate_strings(10, 100_000, 1);
    let mut stores: [Box<dyn KvsEngine>; 3] = [
        Box::new(KvStoreV2::open(temp_dir_1.path()).unwrap()),
        Box::new(SledStore::open(temp_dir_2.path()).unwrap()),
        Box::new(MemStore::new()),
    ];

    {
        let mut group_write = c.benchmark_group("read");
        for mut store in stores.iter_mut() {
            group_write.bench_function(
                store.name(),
                |b| {
                    b.iter(|| {
                        for (key, value) in keys.iter().zip(values.iter()) {
                            store.set(key.clone(), value.clone()).unwrap();
                        }
                    })
                }
            );
        }
    }
    {
        let mut group_read = c.benchmark_group("write");
        for store in stores.iter() {
            group_read.bench_function(
                store.name(),
                |b| {
                    b.iter(|| {
                        for key in keys.iter() {
                            store.get(key.clone()).unwrap();
                        }
                    })
                }
            );
        }
    }
}

criterion_group!(benches, bench_write_read);
criterion_main!(benches);
