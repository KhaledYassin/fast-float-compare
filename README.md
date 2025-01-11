# Floating Point Comparison Benchmark

[![Build Status](https://github.com/KhaledYassin/fast-float-compare/actions/workflows/ci.yml/badge.svg)](https://github.com/KhaledYassin/fast-float-compare/actions/workflows/ci.yml)
[![Code Coverage](https://codecov.io/gh/KhaledYassin/fast-float-compare/branch/main/graph/badge.svg)](https://codecov.io/gh/KhaledYassin/fast-float-compare)
[![Crates.io](https://img.shields.io/crates/v/fast-float-compare)](https://crates.io/crates/fast-float-compare)
[![Documentation](https://docs.rs/fast-float-compare/badge.svg)](https://docs.rs/fast-float-compare)
[![License](https://img.shields.io/crates/l/fast-float-compare.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rustc-1.70+-lightgray.svg)](https://blog.rust-lang.org/2023/03/30/Rust-1.70.0.html)

A lightweight experiment comparing different approaches to floating-point number comparison in Rust.

## Objective

This project aims to explore whether a minimal floating-point representation focused solely on comparison operations can outperform the more comprehensive `Decimal` from the `rust_decimal` crate for ordering/comparison use cases.

The hypothesis is that by stripping away all the overhead required for mathematical operations and focusing purely on comparison logic, we might achieve better performance for scenarios where only ordering matters.

## Approach

The project implements two main approaches:

1. Using `Decimal` - A full-featured decimal floating-point implementation
2. Using `Float` - A minimal custom implementation that:
   - Stores numbers as (mantissa, exponent, sign) triplets
   - Only implements comparison operations
   - Skips all the complexity needed for arithmetic operations

## Benchmarks

The project uses Criterion.rs for benchmarking. To run the benchmarks:

```bash
cargo bench
```

This will compare the performance of:

- Comparing numbers using `Decimal`
- Comparing numbers using our minimal `Float` implementation
- Converting between `f64` and `Float`
- Converting between `f64` and `Decimal`

## Usage

```rust

// Create a new Float
let a = Float::from_f64(1.23).unwrap();
let b = Float::from_f64(4.56).unwrap();

// Compare the two numbers
assert!(a < b);

// Convert back if needed
let a_f64 = a.to_f64();
let b_f64 = b.to_f64();

```

## Project Structure

- `src/lib.rs` - Core `Float` implementation
- `src/main.rs` - Simple demonstration program
- `benches/float_comparison.rs` - Benchmarking code

## Contributing

This is an experimental project aimed at exploring a specific performance hypothesis. Feel free to:

- Run the benchmarks on your own hardware
- Suggest optimizations to the minimal implementation
- Add additional comparison approaches

## License

MIT License

## Acknowledgments

Thanks to the `rust_decimal` crate for providing a great reference implementation for decimal floating-point numbers in Rust.
