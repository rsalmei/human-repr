use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanRepr;
use std::fmt::Write;

fn benchmark<T: HumanRepr>(val: T, buf: &mut String) {
    let _ = write!(buf, "{}", black_box(val).human_duration());
    buf.clear();
}

pub fn small(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-duration 2ns", |b| {
        b.iter(|| benchmark(0.0000000024, &mut buf))
    });
}

pub fn medium(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-duration 2ms", |b| {
        b.iter(|| benchmark(0.0244432, &mut buf))
    });
}

pub fn big(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-duration 2m", |b| b.iter(|| benchmark(123, &mut buf)));
}

criterion_group!(benches, small, medium, big);
criterion_main!(benches);
