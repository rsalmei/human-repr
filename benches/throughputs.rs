use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanThroughput;
use std::fmt::Write;

fn benchmark<T: HumanThroughput>(val: T, buf: &mut String) {
    let _ = write!(buf, "{}", black_box(val).human_throughput_bytes());
    buf.clear();
}

pub fn small(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-throughput", |b| {
        b.iter(|| benchmark(1. / (60. * 60. * 24.), &mut buf))
    });
}

criterion_group!(benches, small);
criterion_main!(benches);
