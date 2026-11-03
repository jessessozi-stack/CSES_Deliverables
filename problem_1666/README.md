# CSES 1666: Building Roads (Rust Implementation)

This project contains the solution, dual-algorithm implementations, integration test suite, and performance benchmark report for the **Building Roads** problem from the CSES Problem Set.

## Project Structure

```text
problem_1666/
├── Cargo.toml             # Rust package configuration & dependencies
├── benchmark.md           # Standalone performance analysis report
├── README.md              # Project documentation and usage guide
└── src/
    ├── main.rs            # Algorithm 1: DFS Connected Components & Binary Entry Point
    └── alternative.rs     # Algorithm 2: Disjoint Set Union (DSU) / Union-Find
```

Implementations (Dual Algorithms)
To satisfy examination requirements, this task implements and compares two distinct strategies for identifying and bridging disconnected components:

1. Algorithm 1: DFS Connected Components (src/main.rs)
Strategy: Constructs an adjacency list from the input road network and uses recursive Depth-First Search to traverse components, identifying unique representative leaders to bridge together.

Advantages: Clean representation of graph connectivity using standard recursive traversal patterns.

2. Algorithm 2: Disjoint Set Union (DSU) (src/alternative.rs)
Strategy: Manages disjoint sets with path compression, dynamically merging node groups as edges are processed and linking isolated sets to a common root.

Advantages: Highly efficient set management with minimal memory overhead and flat tree structures.

Testing and Micro-Benchmarking
The project includes official CSES integration tests (tests/cses_tests.rs) and an inline micro-benchmark suite (bench_tests) to verify correctness and evaluate runtime performance.

Running Tests
To run all verification tests and view live micro-benchmark durations:

```
Bash
cargo test -- --nocapture
```

