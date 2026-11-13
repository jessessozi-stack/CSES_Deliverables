# Problem 1648: Dynamic Range Sum Queries

This project implements and compares two data structures for efficiently handling
dynamic range sum queries with point updates:

- **Fenwick Tree (Binary Indexed Tree)** - Algorithm 1
- **Segment Tree** - Algorithm 2

The goal is to support two types of operations on an array of n elements:
1. Point updates (change a value at a specific index)
2. Range sum queries (compute the sum of elements in a range [a, b])

## How the Task Works

### Input Format
The program reads from standard input in the following format:
```
n q
arr[1] arr[2] ... arr[n]
q lines of queries
```

Each query has the form:
- `1 k u` - Point update: set `arr[k] = u`
- `2 a b` - Range query: compute sum of `arr[a] + arr[a+1] + ... + arr[b]`

### Execution Flow
1. Read n (array size) and q (number of queries)
2. Read the initial array of n elements
3. Build the chosen data structure (Fenwick Tree or Segment Tree)
4. Process each of the q queries:
   - For type 1: update the data structure with the new value
   - For type 2: query the data structure for the range sum and output the result
5. Print all query results

### Test Suites
- **Micro-benchmark**: Runs both algorithms on the same input and verifies they produce identical results
- **Integration tests**: Test with predefined input/output files (tests/inout/1.in, tests/inout/2.in)

---

## Fenwick Tree (Binary Indexed Tree) - Algorithm 1

### How It Works

The Fenwick Tree is a data structure that efficiently maintains prefix sums
with the following operations:
- **Point update**: Add a delta value to position k
- **Prefix query**: Compute sum of elements from position 1 to k
- **Range query**: Compute sum from position a to b = prefix(b) - prefix(a-1)

### Internal Structure

The tree uses a flat `Vec<i64>` of size `n + 1` (1-indexed). Each element at index `i`
stores the sum of a specific range of the original array, determined by the least
significant bit (LSB) of `i`.

### Key Operations

#### Update (`bit.update(idx, delta)`)
```
while idx <= n:
    bit[idx] += delta
    idx += idx & -idx    // Add LSB to move to next responsible index
```

The expression `idx & -idx` isolates the least significant bit,
allowing the tree to efficiently jump to the next index that needs updating.

#### Query (`bit.query(idx)`)
```
sum = 0
while idx > 0:
    sum += bit[idx]
    idx -= idx & -idx    // Subtract LSB to move to parent index
```

This accumulates contributions from all relevant tree nodes to compute the prefix sum.

#### Range Query (`bit.range_query(a, b)`)
```
return bit.query(b) - bit.query(a - 1)
```

### Complexity
- **Time**: O(log n) for both update and query
- **Space**: O(n) - requires exactly n+1 elements
- **Advantages**: Small constant factor, excellent cache locality, simple implementation

### Why It's Faster in Practice
- Only requires bitwise operations (`&`, `-`) which are cheap CPU instructions
- Flat array means better cache locality than tree-based structures
- Fewer memory accesses per operation

---

## Segment Tree - Algorithm 2

### How It Works

The Segment Tree is a binary tree data structure where each node represents
a segment (subrange) of the original array. The tree supports efficient
range queries and point updates.

### Internal Structure

The tree is stored in a `Vec<i64>` of size `2 * n` (where `n` is the next power of 2 >= array size).
- Leaves (positions n to 2n-1) store the actual array values
- Internal nodes store the sum of their children

### Key Operations

#### Build (`seg_tree.build(arr)`)
1. Place array values at tree leaves: `tree[n + i] = arr[i]`
2. Build internal nodes bottom-up: `tree[i] = tree[2*i] + tree[2*i + 1]` for i from n-1 down to 1

#### Update (`seg_tree.update(idx, val)`)
```
idx += n                    // Move to leaf position
tree[idx] = val             // Update the leaf
idx /= 2                    // Move to parent
while idx > 0:
    tree[idx] = tree[2*idx] + tree[2*idx + 1]  // Update internal node
    idx /= 2
```

#### Query (`seg_tree.query(l, r)`)
```
l += n
r += n
sum = 0
while l <= r:
    if l is odd:      // l is right child
        sum += tree[l]
        l++
    if r is even:     // r is left child
        sum += tree[r]
        r--
    l /= 2
    r /= 2
return sum
```

The query algorithm walks up the tree from both ends, collecting contributions
from nodes that perfectly cover the query range.

### Complexity
- **Time**: O(log n) for both update and query
- **Space**: O(n) (typically 4n allocated for safety, or 2*next_pow2(n) used here)
- **Advantages**: Very flexible, can support other range queries (min, max, count)

### Why It's Slower in Practice
- More memory accesses per operation (tree structure scattered in memory)
- Integer division (`/ 2`) required for tree navigation
- Larger memory footprint (2 * next_power_of_two(n) elements)
- More complex query logic with multiple conditional branches

---

## Comparison: Fenwick Tree vs Segment Tree

| Aspect | Fenwick Tree | Segment Tree |
|--------|-------------|--------------|
| **Complexity** | O(log n) | O(log n) |
| **Code Complexity** | Simple (5-7 lines per op) | Moderate (15-20 lines per op) |
| **Memory** | O(n) minimal | O(n) with factor ~2-4 |
| **Constant Factor** | Very low | Moderate |
| **Flexibility** | Limited to prefix sums | High (min, max, sum, range updates) |
| **Cache Locality** | Excellent | Good, but scattered |
| **This Benchmark** | **213ms** | **273ms** (~22% slower) |

---

## How the Benchmark Works

The micro-benchmark in `src/main.rs` (`bench_tests` module) performs:

1. **Input Generation**:
   - Creates n = 50,000 elements (values 1 through n)
   - Creates q = 50,000 alternating queries:
     - Even-indexed queries: Point updates (arr[k] = i)
     - Odd-indexed queries: Range sum queries (random ranges)

2. **Execution**:
   - Runs Algorithm 1 (Fenwick Tree) and measures elapsed time
   - Runs Algorithm 2 (Segment Tree) and measures elapsed time
   - Verifies both produce identical results

3. **Output**:
   ```
   --- MICRO-BENCHMARK: DYNAMIC RANGE SUM QUERIES ---
   Algorithm 1 (Fenwick Tree): 213.44ms
   Algorithm 2 (Segment Tree): 273.57ms
   -----------------------------------------------
   ```

---

## Code Structure

### `src/main.rs`
- **`FenwickTree` struct**: Implements update, query, and range_query operations
- **`solve(input: &str) -> String`**: Main function that reads input and processes queries using Fenwick Tree
- **`alternative::solve(input: &str) -> String`**: Same interface using Segment Tree (defined in `src/alternative.rs`)
- **`bench_tests` module**: Micro-benchmark that runs both algorithms and compares results

### `src/alternative.rs`
- **`SegmentTree` struct**: Implements build, update, and query operations
- **`solve(input: &str) -> String`**: Same interface as main solve but uses Segment Tree

### `tests/cses_tests.rs`
- Two integration tests with predefined inputs and expected outputs
- Verifies correctness of both algorithms on standard test cases

---

## Usage

```bash
# Build the project
cargo build

# Run all tests (includes micro-benchmark)
cargo test

# Run with sample input
./target/debug/problem_1648 < tests/inout/1.in

# Run benchmark specifically
cargo test run_micro_benchmark -- --nocapture
```
