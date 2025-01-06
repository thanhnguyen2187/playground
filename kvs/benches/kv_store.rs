use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kvs::{KvStoreV2, KvsEngine};
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng, SeedableRng,
};
use tempfile;

pub fn bench_kvs(c: &mut Criterion) {
    let temp_dir = tempfile::tempdir().unwrap();
    let mut store = KvStoreV2::open(temp_dir.path()).unwrap();

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut keys: Vec<String> = Vec::new();
    c.bench_function("kvs_write", |b| {
        b.iter(|| {
            // for _ in 0..100 {
            let key_len = rng.gen_range(1..100_000);
            let key: String = Alphanumeric.sample_string(&mut rng, key_len);
            keys.push(key.clone());
            let value_len = rng.gen_range(1..100_000);
            let value: String = Alphanumeric.sample_string(&mut rng, value_len);
            store.set(key, value).unwrap();
            // }
        })
    });

    c.bench_function("kvs_read", |b| {
        b.iter(|| {
            for key in &keys {
                store.get(key.clone()).unwrap();
            }
        })
    });
}

pub fn bench_sled(c: &mut Criterion) {
    let temp_dir = tempfile::tempdir().unwrap();
    let mut store = KvStoreV2::open(temp_dir.path()).unwrap();

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut keys: Vec<String> = Vec::new();
    c.bench_function("sled_write", |b| {
        b.iter(|| {
            // for _ in 0..100 {
            let key_len = rng.gen_range(1..100_000);
            let key: String = Alphanumeric.sample_string(&mut rng, key_len);
            keys.push(key.clone());
            let value_len = rng.gen_range(1..100_000);
            let value: String = Alphanumeric.sample_string(&mut rng, value_len);
            store.set(key, value).unwrap();
            // }
        })
    });

    c.bench_function("sled_read", |b| {
        b.iter(|| {
            for key in &keys {
                store.get(key.clone()).unwrap();
            }
        })
    });
}

criterion_group!(benches, bench_kvs, bench_sled);
criterion_main!(benches);
