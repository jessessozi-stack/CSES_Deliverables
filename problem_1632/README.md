# CSES 1632: Movie Festival II (Rust Implementation)

This project contains the solution, dual-algorithm implementations, integration test suite, and performance benchmark report for the **Movie Festival II** problem from the CSES Problem Set.

## Project Structure

```text
problem_1632/
├── Cargo.toml             # Rust package configuration & dependencies
├── benchmark.md           # Standalone performance analysis report
├── README.md              # Project documentation and usage guide
└── src/
    ├── main.rs            # Algorithm 1: BTreeSet Multiset & Binary Entry Point
    └── alternative.rs     # Algorithm 2: Sorted Vector Simulation
```

##Implementations (Dual Algorithms)
**1. Algorithm 1: BTreeSet Multiset (src/main.rs)
Strategy: Maintains member end times in a BTreeSet indexed with unique IDs. Uses .range(..=query).next_back() to find the optimal member in logarithmic time.

Advantages: True $O(\log K)$ multiset operations.2. 

**2. Algorithm 2: Sorted Vector Simulation (src/alternative.rs)
Strategy: Stores member end times in a sorted vector, utilizing binary search (partition_point) and element shifting.

Advantages: Leverages contiguous memory layout for search queries.

##Testing and Micro-Benchmarking
To run all verification tests and view live micro-benchmark durations:

```
Bash
cargo test -- --nocapture
```
