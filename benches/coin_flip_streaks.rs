use coin_flip_streaks::experiment;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("coin flip streaks", |b| b.iter(|| experiment()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);