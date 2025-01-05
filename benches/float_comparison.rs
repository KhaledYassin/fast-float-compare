use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_float_compare::Float;
use rand::Rng;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

fn generate_test_numbers(count: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(-1000.0..1000.0)).collect()
}

fn benchmark_comparisons(c: &mut Criterion) {
    let numbers = generate_test_numbers(1000);

    let decimals: Vec<Decimal> = numbers
        .iter()
        .map(|&n| Decimal::from_f64(n).unwrap())
        .collect();

    let raw_floats: Vec<Float> = numbers
        .iter()
        .map(|&n| Float::from_f64(n).unwrap())
        .collect();

    c.bench_function("decimal_comparison", |b| {
        b.iter(|| {
            for i in 0..decimals.len() - 1 {
                black_box(decimals[i].cmp(&decimals[i + 1]));
            }
        })
    });

    c.bench_function("raw_float_comparison", |b| {
        b.iter(|| {
            for i in 0..raw_floats.len() - 1 {
                black_box(raw_floats[i].cmp(&raw_floats[i + 1]));
            }
        })
    });
}

fn benchmark_conversion(c: &mut Criterion) {
    let numbers = generate_test_numbers(1000);

    let decimals: Vec<Decimal> = numbers
        .iter()
        .map(|&n| Decimal::from_f64(n).unwrap())
        .collect();

    let raw_floats: Vec<Float> = numbers
        .iter()
        .map(|&n| Float::from_f64(n).unwrap())
        .collect();

    c.bench_function("float_to_decimal", |b| {
        b.iter(|| {
            for &n in &numbers {
                black_box(Decimal::from_f64(n).unwrap());
            }
        })
    });

    c.bench_function("float_to_raw_float", |b| {
        b.iter(|| {
            for &n in &numbers {
                black_box(Float::from_f64(n));
            }
        })
    });

    c.bench_function("decimal_to_float", |b| {
        b.iter(|| {
            for &n in &decimals {
                black_box(Decimal::to_f64(&n).unwrap());
            }
        })
    });

    c.bench_function("raw_float_to_float", |b| {
        b.iter(|| {
            for &n in &raw_floats {
                black_box(n.to_f64());
            }
        })
    });
}

criterion_group!(benches, benchmark_comparisons, benchmark_conversion);
criterion_main!(benches);
