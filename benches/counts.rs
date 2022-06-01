use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanRepr;

pub fn benchmark_small(c: &mut Criterion) {
    c.bench_function("human-count 2B", |b| {
        b.iter(|| black_box(2_u64).human_count_bytes())
    });
}

pub fn benchmark_medium(c: &mut Criterion) {
    c.bench_function("human-count 20MB", |b| {
        b.iter(|| black_box(23433454432_u64).human_count_bytes())
    });
}

pub fn benchmark_big(c: &mut Criterion) {
    c.bench_function("human-count 200TB", |b| {
        b.iter(|| black_box(234334544434332_u64).human_count_bytes())
    });
}

criterion_group!(benches, benchmark_small, benchmark_medium, benchmark_big);
criterion_main!(benches);
