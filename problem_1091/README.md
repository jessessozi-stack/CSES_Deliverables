# CSES 1091: Concert Tickets (Rust Implementation)

This project contains the solution, dual-algorithm implementations, integration test suite, and performance benchmark report for the **Concert Tickets** problem from the CSES Problem Set.

## Project Structure

```text
problem_1091/
├── Cargo.toml             # Rust package configuration & dependencies
├── benchmark.md           # Standalone performance analysis report
├── README.md              # Project documentation and usage guide
└── src/
    ├── main.rs            # Algorithm 1: BTreeMap Frequency Map & Binary Entry Point
    └── alternative.rs     # Algorithm 2: Indexed BTreeSet Multiset
```

## Implementations (Dual Algorithms)
1. Algorithm 1: BTreeMap Frequency Map (src/main.rs)
*Strategy: Maps unique ticket prices to their available counts. Uses .range(..=max_price).next_back() to find the highest valid price in logarithmic time.

*Advantages: Compact tree structure with reduced node allocations for duplicate prices.

2. Algorithm 2: Indexed BTreeSet (src/alternative.rs)
*Strategy: Inserts tickets as unique (price, index) tuples into a BTreeSet, naturally differentiating identical prices.

*Advantages: Clean modeling of multiset elements as distinct entities.

## Testing and Micro-Benchmarking
To run all verification tests and view live micro-benchmark durations:

```
Bash
cargo test -- --nocapture
```

## Environment and Toolchain
*Language: Rust (Stable)

*Development Environment: Ubuntu Linux (via Oracle VirtualBox)

*Version Control: Git / GitHub Monorepo (CSES_Deliverables)
