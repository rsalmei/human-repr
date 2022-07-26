use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanDuration;
use std::fmt::Write;

fn benchmark<T: HumanDuration>(val: T, buf: &mut String) {
    let _ = write!(buf, "{}", black_box(val).human_duration());
    buf.clear();
}

pub fn small(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-duration", |b| {
        b.iter(|| benchmark(0.000000001_f64, &mut buf))
    });
}

criterion_group!(benches, small);
criterion_main!(benches);
