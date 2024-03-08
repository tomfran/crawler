use crawler::worker::simhash::simhash;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{distributions::Alphanumeric, Rng};

fn simhash_bench(c: &mut Criterion) {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10_000)
        .map(char::from)
        .collect();

    c.bench_function("simhash", |b| b.iter(|| simhash(black_box(&s), 3)));
}

criterion_group!(benches, simhash_bench);
criterion_main!(benches);
