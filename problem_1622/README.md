Creating Strings

# CSES 1622: 

## Project Structure

---

```text
problem_1622/
├── Cargo.toml             # Rust dependencies
├── benchmark.md           # Standalone performance analysis report
├── README.md              # Project documentation and usage guide
└── src/
    ├── main.rs            # Algorithm 1: Frequency Map Backtracking & Binary Entry Point
    └── alternative.rs     # Algorithm 2: Permutation Swapping with BTreeSet Deduplication
````

Implementations (Dual Algorithms)
To satisfy examination requirements, this task implements and compares two distinct algorithmic strategies for generating unique string permutations in lexicographical order:

1. Algorithm 1: Frequency Map Backtracking (src/main.rs)
Strategy: Counts the frequency of each lowercase English character (a-z) using a fixed array. It recursively builds permutations by consuming available character counts.

Advantages: Naturally prunes invalid states, completely avoids duplicate permutations on-the-fly, and minimizes heap allocations. This serves as the primary production-grade solution.

2. Algorithm 2: Permutation Swapping & BTreeSet (src/alternative.rs)
Strategy: Generates all raw permutations recursively via element swapping and inserts them into a BTreeSet<String>.

Advantages: Simpler brute-force mental model; the set automatically handles alphabetical sorting and duplicate removal, though at a higher CPU and memory cost.

Testing & Micro-Benchmarking
The project includes both automated CSES integration tests and an inline micro-benchmark suite (bench_tests) to verify correctness and evaluate performance.

Running Tests
To run all verification tests (including CSES test vectors and the micro-benchmark output):

Bash
cargo test -- --nocapture

Benchmark Summary (from benchmark.md)
Running a micro-benchmark on a representative test string (aabacde) yields the following performance profile:
Algorithm 1 (Frequency Map Backtracking): ~442 µs (Highly optimized, zero redundant allocations).
Algorithm 2 (BTreeSet Swapping): ~4.03 ms (Slower due to overhead from tree-based insertions and duplicate generation).

Environment & Toolchain
Language: Rust (Stable)
Development Environment: Ubuntu Linux (via Oracle VirtualBox)
Version Control: Git / GitHub Monorepo (CSES_Deliverables)
