# Benchmark Report: Missing Number (CSES 1083)

## Overview
This report analyzes and compares two distinct algorithmic strategies implemented in Rust for finding the single missing integer in an unsorted sequence from 1 to N:
1. **Algorithm 1**: Mathematical Sum Formula (`main.rs`).
2. **Algorithm 2**: Dynamic Vector Allocation and Sorting (`alternative.rs`).

---

## Methodology & Execution Environment
* **Environment**: Ubuntu Linux running within Oracle VirtualBox.
* **Toolchain**: Rust stable (via Cargo) with debug/test build profiles.
* **Test Vector**: A generated sequence of size $N = 200,000$ with a single element omitted in the middle.
* **Measurement Tool**: High-resolution execution timing using Rust's `std::time::Instant`.

---

## Performance Analysis

### 1. Time Complexity & Execution Speed
* **Algorithm 1 (Mathematical Sum Formula)**: 
  * Computes the expected sum of integers from 1 to $N$ using the closed-form arithmetic progression formula: $N \times (N + 1) / 2$.
  * Iterates through the input tokens once to accumulate the running sum, resulting in an optimal $O(N)$ time complexity.
  * **Result**: Extremely lightweight CPU instructions with minimal branching.

* **Algorithm 2 (Dynamic Vector Sorting)**: 
  * Parses tokens into a heap-allocated dynamic array (`Vec<usize>`), then sorts the vector using unstable sorting (`sort_unstable()`).
  * Scrapes sorted elements sequentially to find the missing index gap, yielding an $O(N \log N)$ time complexity due to the sorting overhead.
  * **Result**: While modern CPU branch prediction and cache lines make sorting remarkably fast for $N = 200,000$, its asymptotic scalability degrades faster as $N$ grows larger.

### 2. Memory Overhead & Space Complexity
* **Algorithm 1**: $O(1)$ auxiliary space complexity. It operates entirely through stream processing and scalar addition without allocating heap memory for collections.
* **Algorithm 2**: $O(N)$ auxiliary space complexity. It requires allocating a dynamic vector on the heap to store all $N-1$ elements before sorting can occur.

---

## Conclusion
While both implementations comfortably pass all 14 official CSES test vectors, **Algorithm 1 (Mathematical Sum Formula)** is the definitive production choice. It guarantees constant space efficiency and superior scaling by eliminating both heap allocations and sorting overhead.
