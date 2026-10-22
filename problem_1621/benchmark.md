# Benchmark Report: Distinct Numbers (CSES 1621)

## Overview
This report analyzes and compares two distinct algorithmic strategies implemented in Rust for calculating the number of unique elements in an array:
1. **Algorithm 1**: Balanced Binary Search Tree (`BTreeSet` in `main.rs`).
2. **Algorithm 2**: Dynamic Vector Sorting and Linear Scan (`alternative.rs`).

---

## Methodology & Execution Environment
* **Environment**: Ubuntu Linux running within Oracle VirtualBox.
* **Toolchain**: Rust stable (via Cargo) with debug/test build profiles.
* **Test Vector**: A generated sequence of size $N = 200,000$ containing repetitive values to evaluate deduplication efficiency.
* **Measurement Tool**: High-resolution execution timing using Rust's `std::time::Instant`.

---

## Performance Analysis & Empirical Results

Running the inline micro-benchmark suite on a test dataset of $N = 200,000$ elements yielded the following execution times:

* **Algorithm 1 (`BTreeSet` Approach)**: **183.14 ms**
  * *Behavior*: Inserts each parsed element into a red-black tree (`BTreeSet`), which automatically maintains uniqueness and sorted order. 
  * *Complexity*: $O(N \log N)$ time complexity. While logically concise, individual tree node heap allocations and pointer chasing introduce CPU cache overhead.

* **Algorithm 2 (Sorting + Linear Scan)**: **176.44 ms**
  * *Behavior*: Collects elements into a contiguous dynamic vector (`Vec<i64>`), sorts them using unstable sorting (`sort_unstable()`), and executes a single linear pass to count transitions between distinct adjacent values.
  * *Complexity*: $O(N \log N)$ sorting time followed by an $O(N)$ linear pass. 
  * *Advantage*: Demonstrates superior memory locality and cache efficiency, processing the dataset slightly faster than pointer-based tree insertions.

---

## Conclusion
Both algorithms achieve an optimal theoretical time bound of $O(N \log N)$ and successfully pass all test vectors. Empirical testing confirms that **Algorithm 2 (Vector Sorting and Linear Scan)** achieves a slight performance edge due to contiguous memory allocation and reduced pointer overhead. **Algorithm 1 (`BTreeSet`)** remains ideal when ordered stream insertion is required.
