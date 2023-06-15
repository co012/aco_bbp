use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn pow_native(n: usize) {
    for _ in 0..n {
        for a in [0.5f64, 1.0, 1.1, 1.2, 3.0, 6.4] {
            for x in [0.5f64, 1.0, 1.1, 1.2, 3.0, 6.4] {
                a.powf(x);
            }
        }
    }
}

fn pow_unsafe(n: usize) {
    for _ in 0..n {
        for a in [0.5f64, 1.0, 1.1, 1.2, 3.0, 6.4] {
            for x in [1i32, 2, 3, 4, 5, 6] {
                a.powi(x);
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pow native", |b| b.iter(|| pow_native(black_box(10000))));
    c.bench_function("pow int", |b| b.iter(|| pow_unsafe(black_box(10000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);