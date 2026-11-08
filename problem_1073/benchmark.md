# Benchmark: Towers Algorithm Comparison

This project benchmarks two algorithms that solve the **Towers problem** (also related to patience sorting and the minimum number of increasing subsequences). Both algorithms process a sequence of cube sizes and determine the minimum number of towers needed such that each tower has cubes in non-increasing order from bottom to top.

## Algorithms

### Algorithm 1: Sorted Vector + Binary Search (`main.rs`)

**Implementation**: Uses a `Vec<i32>` to maintain tower top heights, with `partition_point` for binary search.

**Strategy**:
- For each cube, find the smallest tower top that is strictly greater than the cube using `partition_point(|&x| x <= cube)`.
- If such a tower top exists, replace it with the current cube (placing the cube on that tower).
- If no such tower top exists (all tower tops are <= cube), start a new tower by pushing the cube.

**Time Complexity**: O(n log n) per element for the binary search, but note that `partition_point` on a `Vec` involves shifting elements when replacing, which can make the worst case O(n²) depending on the data distribution. However, in practice the binary search portion is very efficient.

**Space Complexity**: O(n) — stores at most n tower top heights in the vector.

**Key Characteristics**:
- Contiguous memory allocation (cache-friendly)
- Simple integer comparisons, no heap allocations during the main loop
- The compiler can heavily optimize the binary search loop
- `partition_point` is a stable, optimized standard library operation

### Algorithm 2: BTreeMap Frequency Map (`alternative.rs`)

**Implementation**: Uses a `BTreeMap<i32, i32>` mapping tower top heights to their counts.

**Strategy**:
- For each cube, search the BTreeMap for the smallest key strictly greater than the cube using `range((Bound::Excluded(cube), Bound::Unbounded))`.
- If found and its count is 1, remove the entry; otherwise decrement the count.
- Increment the count for the current cube's height in the map.
- Track total towers separately.

**Time Complexity**: O(n log n) — each element involves BTreeMap operations (split, find, insert/remove), each O(log n) in a red-black tree.

**Space Complexity**: O(n) — stores at most n distinct (height, count) pairs in the worst case.

**Key Characteristics**:
- Node-based allocation (pointer chasing, less cache-friendly)
- Tree restructuring operations (rotations) during insert/remove
- Additional overhead from `Bound` enum and boundary checks
- Frequency tracking adds bookkeeping but avoids duplicate entries

## Benchmark Results

The micro-benchmark was run with `n = 50,000` synthetic test cases (cube sizes generated with `(i * 37) % 100000`), using the debug build configuration.

| Algorithm | Execution Time | Relative Speed |
|-----------|---------------|----------------|
| Algorithm 1 (Sorted Vector + Binary Search) | ~104 ms | Baseline |
| Algorithm 2 (BTreeMap Frequency Map) | ~368 ms | ~3.5x slower |

**Test Command**: `cargo test run_micro_benchmark -- --nocapture`

```
--- MICRO-BENCHMARK: TOWERS ---
Algorithm 1 (Sorted Vector + Binary Search): 104.181108ms
Algorithm 2 (BTreeMap Frequency Map): 367.992853ms
---------------------------------------
```

## Interpretation of Results

### Why is Algorithm 1 significantly faster despite both being O(n log n)?

1. **Cache Locality**: Algorithm 1's `Vec<i32>` stores data in a contiguous memory block. The binary search walks through this array with predictable access patterns, which is highly CPU-cache friendly. Algorithm 2's `BTreeMap` involves pointer chasing between heap-allocated nodes, which defeats CPU prefetchers and incurs cache misses.

2. **Allocation Overhead**: Algorithm 1 only allocates the initial vector (capacity n). Algorithm 2's `BTreeMap` performs numerous heap allocations for tree nodes as elements are inserted and removed, adding both allocation time and garbage collection pressure (even in Rust, the raw allocator has overhead).

3. **Simpler Operations**: `partition_point` on a `Vec` is a tight loop of integer comparisons and index arithmetic. `BTreeMap` operations involve:
   - `range()` with `Bound` enum discrimination
   - Tree navigation with parent/child pointers
   - Potential node rotations maintaining red-black tree invariants
   - `Bound` checking and option unwrapping

4. **Compiler Optimizations**: The Rust compiler can heavily optimize the binary search on a `Vec`, including loop unrolling, vectorization, and predictable memory access patterns. Tree-based structures are harder for the compiler to optimize at the same level.

5. **Frequency Map Diminishing Returns**: While Algorithm 2's frequency map (tracking counts of shared tower tops) reduces some operations, the overhead of maintaining the BTreeMap structure far outweighs the benefit of fewer insert/remove operations. In this dataset, the frequency optimization doesn't compensate for the tree overhead.

### When Would Algorithm 2 Be Preferable?

- When you need **ordered iteration** over tower top heights (BTreeMap provides sorted keys by default).
- When you need **concurrent access** (BTreeMap has no std feature, but `fairbriel` or other concurrent BTree variants exist).
- When you need **additional metadata** per tower top that isn't just a count.
- When the number of distinct tower top heights is very small compared to n, the frequency map might provide more benefit.

### Practical Takeaway

For this specific problem (computing the minimum number of towers/subsequences), **Algorithm 1 is the clear winner**. The sorted vector + binary search approach leverages Rust's strengths—low-level control with high-level abstractions—resulting in code that is both efficient and easy to understand. The BTreeMap approach, while more flexible in terms of what it can represent, incurs significant overhead that makes it unsuitable for performance-critical instances of this problem.

## Complexity Summary

| Metric | Algorithm 1 | Algorithm 2 |
|--------|-------------|-------------|
| Time Complexity | O(n log) average, O(n²) worst case (due to Vec shifts) | O(n log n) guaranteed |
| Space Complexity | O(n) | O(n) |
| Cache Locality | Excellent (contiguous Vec) | Poor (node-based BTreeMap) |
| Allocation Rate | Low (one vector allocation) | High (many BTreeMap node allocations) |
| Constant Factor | Small | Large |
