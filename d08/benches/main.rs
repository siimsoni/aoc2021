use criterion::{criterion_group, criterion_main, Criterion};
use d08::{parser, part1, part2};
use std::io::{stdin, Read};

fn main_benchmark(c: &mut Criterion) {
    let mut buffer = String::new();
    if let Ok(_) = stdin().lock().read_to_string(&mut buffer) {
        c.bench_function("parse", |b| {
            b.iter(|| parser::parse(buffer.as_bytes()));
        });
        let parsed = parser::parse(buffer.as_bytes());
        c.bench_function("p1", |b| {
            b.iter(|| part1::solve(&parsed));
        });
        c.bench_function("p2", |b| b.iter(|| part2::solve(&parsed)));
    }
}

criterion_group!(benches, main_benchmark);
criterion_main!(benches);
