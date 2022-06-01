use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanRepr;

pub fn benchmark_small(c: &mut Criterion) {
    c.bench_function("human-throughput 2/d", |b| {
        b.iter(|| black_box(0.000023148148148).human_throughput_bytes())
    });
}

pub fn benchmark_medium(c: &mut Criterion) {
    c.bench_function("human-throughput 2/h", |b| {
        b.iter(|| black_box(0.000555555555556).human_throughput_bytes())
    });
}

pub fn benchmark_big(c: &mut Criterion) {
    c.bench_function("human-throughput 2/m", |b| {
        b.iter(|| black_box(0.033333333333333).human_throughput_bytes())
    });
}

criterion_group!(benches, benchmark_small, benchmark_medium, benchmark_big);
criterion_main!(benches);
