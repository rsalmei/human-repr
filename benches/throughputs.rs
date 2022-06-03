use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanRepr;
use std::fmt::Write;

fn benchmark<T: HumanRepr>(val: T, buf: &mut String) {
    let _ = write!(buf, "{}", black_box(val).human_throughput_bytes());
    buf.clear();
}

pub fn small(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-throughput 2/d", |b| {
        b.iter(|| benchmark(0.000023148148148, &mut buf))
    });
}

pub fn medium(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-throughput 2/h", |b| {
        b.iter(|| benchmark(0.000555555555556, &mut buf))
    });
}

pub fn big(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-throughput 2/m", |b| {
        b.iter(|| benchmark(0.033333333333333, &mut buf))
    });
}

criterion_group!(benches, small, medium, big);
criterion_main!(benches);
