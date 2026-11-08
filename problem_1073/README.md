# problem_1073 - Towers Algorithm Benchmark

## Problem Description

This project solves the **Towers problem**: given a sequence of cube sizes, determine the minimum number of towers needed such that each tower has cubes in non-increasing order from bottom to top.

Equivalently, this is the problem of partitioning a sequence into the minimum number of non-increasing subsequences, which by Dilworth's theorem equals the length of the longest increasing subsequence.

**Input format:**
- First line: integer `n` (number of cubes)
- Second line: `n` integers representing cube sizes

**Output:** Single integer - the minimum number of towers required.

Both implementations read from stdin and write to stdout.

---

## Solutions

### Algorithm 1: Sorted Vector + Binary Search (`src/main.rs`)

**Approach:** Maintains a vector of tower top heights. For each cube, uses `partition_point` to find the smallest tower top that is `<= cube` (i.e., the first tower top strictly greater than the cube when inverted). If found, replaces that tower top with the current cube; otherwise starts a new tower.

**Key design choice:** Uses `towers.partition_point(|&x| x <= cube)` which finds the boundary where cubes could be placed. The condition `x <= cube` means we're looking for a tower top that is small enough to accept the current cube (since towers are non-increasing from bottom to top, a cube can be placed on a tower if its top is >= the new cube).

**Complexity:** O(n log n) average time, O(n) space.

**Why it's fast:** Contiguous `Vec<i32>` memory, cache-friendly binary search, minimal allocations.

```rust
let idx = towers.partition_point(|&x| x <= cube);
if idx < towers.len() {
    towers[idx] = cube;  // Replace tower top
} else {
    towers.push(cube);   // Start new tower
}
```

### Algorithm 2: BTreeMap Frequency Map (`src/alternative.rs`)

**Approach:** Uses a `BTreeMap<i32, i32>` mapping tower top heights to their counts. For each cube, searches for the smallest key strictly greater than the cube using `range((Bound::Excluded(cube), Bound::Unbounded))`. When a tower top is used, its count is decremented; if count reaches 0, the entry is removed. The current cube's height is then added to the map.

**Key design choice:** Frequency tracking (`count` field) allows multiple towers to share the same top height, reducing duplicate entries. Uses `Bound::Excluded(cube)` to find strictly greater tops.

**Complexity:** O(n log n) time, O(n) space.

```rust
if let Some((&top, &count)) = map.range((Bound::Excluded(cube), Bound::Unbounded)).next() {
    if count == 1 {
        map.remove(&top);
    } else {
        map.insert(top, count - 1);
    }
    *map.entry(cube).or_insert(0) += 1;
} else {
    *map.entry(cube).or_insert(0) += 1;
    total_towers += 1;
}
```

---

## Benchmark Results

**Micro-benchmark configuration:**
- Input size: `n = 50,000` cubes
- Test data: Synthetic generated with `(i * 37) % 100000` for i = 1..=n
- Build: Debug configuration (`cargo test`)
- Measurement: `std::time::Instant` around each algorithm call

| Algorithm | Execution Time | Relative Speed |
|-----------|---------------|----------------|
| **Algorithm 1** (Sorted Vector + Binary Search) | **~104 ms** | Baseline |
| **Algorithm 2** (BTreeMap Frequency Map) | **~368 ms** | ~3.5x slower |

**Raw benchmark output:**
```
--- MICRO-BENCHMARK: TOWERS ---
Algorithm 1 (Sorted Vector + Binary Search): 104.181108ms
Algorithm 2 (BTreeMap Frequency Map): 367.992853ms
---------------------------------------
```

**Test command:** `cargo test run_micro_benchmark -- --nocapture`

---

## Interpretation

Despite both algorithms being O(n log n), Algorithm 1 is significantly faster (~3.5x) due to:

1. **Cache Locality:** `Vec<i32>` stores data contiguously in memory, enabling CPU cache-friendly access patterns during binary search. `BTreeMap` uses node-based heap allocation with pointer chasing, which defeats CPU prefetchers and causes frequent cache misses.


