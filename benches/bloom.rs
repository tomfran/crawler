use crawler::sieve::bloom::Filter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bloom_add_bench(c: &mut Criterion) {
    let mut bloom = Filter::new(1_000_000, 0.01);
    for i in 0..1_000_000 {
        bloom.add(&(i as u32).to_be_bytes());
    }
    let data = 10_u32.to_be_bytes();

    c.bench_function("bloomfilter add", |b| {
        b.iter(|| bloom.add(black_box(&data)))
    });
}

fn bloom_contains_bench(c: &mut Criterion) {
    let mut bloom = Filter::new(1_000_000, 0.01);
    for i in 0..1_000_000 {
        bloom.add(&(i as u32).to_be_bytes());
    }
    let data = 1_000_001_u32.to_be_bytes();

    c.bench_function("bloomfilter contains", |b| {
        b.iter(|| bloom.contains(black_box(&data)))
    });
}

criterion_group!(benches, bloom_add_bench, bloom_contains_bench);
criterion_main!(benches);
