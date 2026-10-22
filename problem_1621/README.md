# CSES 1621: Distinct Numbers (Rust Implementation)

This project contains the solution, dual-algorithm implementations, integration test suite, and performance benchmark report for the **Distinct Numbers** problem from the CSES Problem Set.

## Project Structure

```text
problem_1621/
├── Cargo.toml             # Rust package configuration & dependencies
├── benchmark.md           # Standalone performance analysis report
├── README.md              # Project documentation and usage guide
└── src/
    ├── main.rs            # Algorithm 1: BTreeSet Deduplication & Binary Entry Point
    └── alternative.rs     # Algorithm 2: Dynamic Vector Sorting & Linear Scan
```
Implementations (Dual Algorithms)
To satisfy examination requirements, this task implements and compares two distinct algorithmic strategies for counting unique elements in an array:

1. Algorithm 1: BTreeSet Deduplication (src/main.rs)
Strategy: Parses input tokens and inserts them into a BTreeSet<i64>. Duplicate elements are automatically discarded upon insertion, and the final set length yields the distinct count.

Advantages: Declarative logic with automatic ordering and uniqueness guarantees.

2. Algorithm 2: Dynamic Vector Sorting and Linear Scan (src/alternative.rs)
Strategy: Collects elements into a dynamic vector (Vec<i64>), sorts the vector using unstable sorting, and executes a linear scan to tally elements that differ from their preceding neighbor.

Advantages: Demonstrates raw array processing and leverages contiguous memory cache locality for optimal performance.

Testing and Micro-Benchmarking
The project includes official CSES integration tests (tests/cses_tests.rs) and an inline micro-benchmark suite (bench_tests) to verify correctness and evaluate runtime performance.

Running Tests
To run all verification tests and view live micro-benchmark durations:

```
Bash
cargo test -- --nocapture
```


