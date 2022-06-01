use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanRepr;

pub fn benchmark_small(c: &mut Criterion) {
    c.bench_function("human-duration 2ns", |b| {
        b.iter(|| black_box(0.0000000024).human_duration())
    });
}

pub fn benchmark_medium(c: &mut Criterion) {
    c.bench_function("human-duration 2ms", |b| {
        b.iter(|| black_box(0.0244432).human_duration())
    });
}

pub fn benchmark_big(c: &mut Criterion) {
    c.bench_function("human-duration 2h", |b| {
        b.iter(|| black_box(7283).human_duration())
    });
}

criterion_group!(benches, benchmark_small, benchmark_medium, benchmark_big);
criterion_main!(benches);
