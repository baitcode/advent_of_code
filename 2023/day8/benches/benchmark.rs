use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day8::execute_one;
use day8::execute_two;

fn criterion_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    c.bench_function("task 1", |b| {
        b.iter(|| execute_one(black_box(&mut contents.clone())))
    });
    c.bench_function("task 2", |b| {
        b.iter(|| execute_two(black_box(&mut contents.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
