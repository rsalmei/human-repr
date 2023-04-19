use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanThroughput;
use std::fmt::Write;

pub struct Void;

impl Write for Void {
    fn write_str(&mut self, _s: &str) -> std::fmt::Result {
        Ok(())
    }
}

fn benchmark<T: HumanThroughput>(val: T) {
    let _ = write!(Void, "{}", black_box(val).human_throughput_bytes());
}

pub fn small(c: &mut Criterion) {
    c.bench_function("human-throughput", |b| {
        b.iter(|| benchmark(1. / (60. * 60. * 24.)))
    });
}

criterion_group!(benches, small);
criterion_main!(benches);
