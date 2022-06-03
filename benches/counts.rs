use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanRepr;
use std::fmt::Write;

fn benchmark<T: HumanRepr>(val: T, buf: &mut String) {
    let _ = write!(buf, "{}", black_box(val).human_count_bytes());
    buf.clear();
}

pub fn small(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-count 2B", |b| b.iter(|| benchmark(2_u64, &mut buf)));
}

pub fn medium(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-count 20MB", |b| {
        b.iter(|| benchmark(23433454432_u64, &mut buf))
    });
}

pub fn big(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-count 200TB", |b| {
        b.iter(|| benchmark(234334544434332_u64, &mut buf))
    });
}

criterion_group!(benches, small, medium, big);
criterion_main!(benches);
