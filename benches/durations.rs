use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanDuration;
use std::fmt::Write;

pub struct Void;

impl Write for Void {
    fn write_str(&mut self, _s: &str) -> std::fmt::Result {
        Ok(())
    }
}

fn benchmark<T: HumanDuration>(val: T) {
    let _ = write!(Void, "{}", black_box(val).human_duration());
}

pub fn small(c: &mut Criterion) {
    c.bench_function("human-duration", |b| b.iter(|| benchmark(0.000000001_f64)));
}

criterion_group!(benches, small);
criterion_main!(benches);
