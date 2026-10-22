# CSES 1131: Tree Diameter (Rust Implementation)

This project contains the solution, dual-algorithm implementations, integration test suite, and performance benchmark report for the **Tree Diameter** problem from the CSES Problem Set.

## Project Structure

```text
problem_1131/
├── Cargo.toml             # Rust package configuration & dependencies
├── benchmark.md           # Standalone performance analysis report
├── README.md              # Project documentation and usage guide
└── src/
    ├── main.rs            # Algorithm 1: Two-Pass Breadth-First Search & Binary Entry Point
    └── alternative.rs     # Algorithm 2: Tree Dynamic Programming (DFS)
```

Implementations (Dual Algorithms)
To satisfy examination requirements, this task implements and compares two distinct linear-time strategies for finding the tree diameter:

1. Algorithm 1: Two-Pass Breadth-First Search (src/main.rs)
Strategy: Performs a BFS from an arbitrary starting node to locate the farthest node (endpoint of the diameter). It then initiates a second BFS from that endpoint to find the maximum distance.

Advantages: Intuitive graph traversal model utilizing queue-based distance tracking.

2. Algorithm 2: Tree Dynamic Programming / DFS (src/alternative.rs)
Strategy: Uses a recursive depth-first search to calculate subtree heights bottom-up, tracking the sum of the two largest child depths at every node.

Advantages: Solves the problem in a single recursive pass, reducing overhead compared to multi-pass queue traversals.

Testing and Micro-Benchmarking
The project includes official CSES integration tests (tests/cses_tests.rs) and an inline micro-benchmark suite (bench_tests) to verify correctness and evaluate runtime performance.

Running Tests
To run all verification tests and view live micro-benchmark durations:

```
Bash
cargo test -- --nocapture
```
