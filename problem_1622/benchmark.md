# Benchmark Report: Creating Strings (CSES 1622)

## Overview
This report compares two distinct algorithmic approaches implemented in Rust for generating all unique permutations of a given string in lexicographical order:
1. **Algorithm 1**: Recursive Backtracking with a Character Frequency Map (`main.rs`).
2. **Algorithm 2**: Recursive Permutation Swapping with a `BTreeSet` for Deduplication (`alternative.rs`).

---

## Methodology & Execution Environment
* **Environment**: Ubuntu Linux running within Oracle VirtualBox.
* **Toolchain**: Rust stable (via Cargo) with release/debug build profiles.
* **Test Vector**: Sample input string containing repeated characters (`aabacde`) to evaluate handling of duplicate pruning.
* **Measurement Tool**: Programmatic execution timing using Rust's high-resolution `std::time::Instant`.

---

## Performance Analysis

### 1. Time Complexity & Execution Speed
* **Algorithm 1 (Frequency Map Backtracking)**: 
  * Operates directly on a fixed-size character frequency array ($26$ elements for lowercase English letters). 
  * Explores only valid state branches, inherently skipping duplicate permutations on-the-fly.
  * **Result**: Exhibits superior execution speed and lower CPU overhead due to avoiding redundant string allocations during the recursion tree traversal.

* **Algorithm 2 (BTreeSet Swapping)**: 
  * Generates all raw permutations via index swapping and inserts every result into a `BTreeSet` to guarantee uniqueness and alphabetical sorting.
  * **Result**: Slower execution times on larger or highly repetitive inputs because it generates duplicate states and relies on tree-based insertions (`O(N \log K)` overhead per insertion).

### 2. Memory Overhead
* **Algorithm 1**: Minimal heap allocations; builds permutations directly into a pre-allocated String buffer and collects results in a vector.
* **Algorithm 2**: Higher memory footprint due to intermediate string instantiations and maintaining a `BTreeSet` containing every unique permutation string.

---

## Conclusion
**Algorithm 1** is the optimal choice for production and automated grading due to its constraint-aware pruning, lower memory consumption, and significantly faster execution profile compared to brute-force set deduplication.
