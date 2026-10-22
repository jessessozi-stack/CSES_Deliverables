# CSES 1083: Missing Number 

This project contains the solution, dual-algorithm implementations, integration test suite, and performance benchmark report for the **Missing Number** problem from the CSES Problem Set.

## Project Structure

```text
problem_1083/
├── Cargo.toml             # Rust package configuration & dependencies
├── benchmark.md           # Standalone performance analysis report
├── README.md              # Project documentation and usage guide
└── src/
    ├── main.rs            # Algorithm 1: Mathematical Sum Formula & Binary Entry Point
    └── alternative.rs     # Algorithm 2: Dynamic Vector Allocation & Sorting
```
Implementations (Dual Algorithms)
To satisfy examination requirements, this task implements and compares two distinct algorithmic strategies for identifying the single missing integer in an unsorted sequence from 1 to N:

1. Algorithm 1: Mathematical Sum Formula (src/main.rs)
   Strategy: Computes the expected sum of integers from 1 to N using the arithmetic progression formula $N \times (N + 1) / 2, then subtracts the accumulated sum of the input sequence.
   Advantages: Achieves optimal O(N) time complexity and O(1) auxiliary space complexity. It operates via stream processing and scalar addition without requiring heap memory allocations or sorting overhead.

2. Algorithm 2: Dynamic Vector Sorting (src/alternative.rs)
   Strategy: Collects input tokens into a heap-allocated dynamic array (Vec<usize>), sorts the vector using unstable sorting, and scans sequentially to locate the missing index gap.
   Advantages: Explicitly demonstrates Rust dynamic vector operations and sorting APIs, operating at O(N \log N) time complexity and O(N) space complexity.

Testing and Micro-Benchmarking
The project includes both official CSES integration tests (tests/cses_tests.rs) and an inline micro-benchmark suite (bench_tests) to verify correctness and evaluate runtime performance.

Running Tests
To run all verification tests and view live micro-benchmark durations:
```
cargo test -- --nocapture
```

Benchmark Summary (from benchmark.md)
Algorithm 1 (Mathematical Sum Formula): Demonstrates superior performance and scaling, avoiding heap allocations and sorting penalties.
Algorithm 2 (Dynamic Vector Sorting): Incurs additional latency and memory overhead as $N$ increases due to dynamic array allocation and sorting operations.   
