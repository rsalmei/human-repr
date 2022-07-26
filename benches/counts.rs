use criterion::{black_box, criterion_group, criterion_main, Criterion};
use human_repr::HumanCount;
use std::fmt::Write;

fn benchmark<T: HumanCount>(val: T, buf: &mut String) {
    let _ = write!(buf, "{}", black_box(val).human_count_bytes());
    buf.clear();
}

pub fn small(c: &mut Criterion) {
    let mut buf = String::with_capacity(64);
    c.bench_function("human-count", |b| b.iter(|| benchmark(1_u32, &mut buf)));
}

criterion_group!(benches, small);
criterion_main!(benches);
