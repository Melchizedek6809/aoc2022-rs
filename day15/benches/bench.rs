use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day15::run_both;
// use lib::euler1; // function to profile

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day15", |b| b.iter(|| run_both(black_box("input.txt"))));
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(20);
    targets = criterion_benchmark
}
criterion_main!(benches);
