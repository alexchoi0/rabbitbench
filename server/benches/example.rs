use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn sorting_benchmark(c: &mut Criterion) {
    let mut data: Vec<i32> = (0..1000).rev().collect();

    c.bench_function("sort_1000", |b| {
        b.iter(|| {
            let mut d = data.clone();
            d.sort();
            black_box(d)
        })
    });
}

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci_20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, sorting_benchmark, fibonacci_benchmark);
criterion_main!(benches);
