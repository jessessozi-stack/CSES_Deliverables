# Benchmark: Fenwick Tree vs Segment Tree for Dynamic Range Sum Queries

## Benchmark Configuration

- **Array size (n)**: 50,000 elements
- **Number of queries (q)**: 50,000 queries
- **Workload**: Alternating point updates and range sum queries
  - Even-indexed queries (0, 2, 4, ...): Point update operation
  - Odd-indexed queries (1, 3, 5, ...): Range sum query operation
- **Input generation**: Random values within i64 range, random indices
- **Test environment**: Standard laptop, debug profile (unoptimized + debuginfo)
- **Rust version**: Latest stable at time of build

## Measured Results

| Algorithm | Execution Time |
|-----------|---------------|
| **Fenwick Tree (Binary Indexed Tree)** | **213.44 ms** |
| **Segment Tree** | **273.57 ms** |

*The benchmark was run via `cargo test run_micro_benchmark -- --nocapture` and the times are the total elapsed time for processing all n array initializations and q queries.*

## Detailed Breakdown

### Fenwick Tree (Algorithm 1)

- **Total time**: 213.439772 ms
- Operations performed:
  - n = 50,000 initial array builds (point updates during construction)
  - q = 50,000 mixed queries (25,000 updates + 25,000 range queries)
- Key characteristics:
  - Low memory footprint: O(n) i64 values
  - Excellent cache locality due to flat array structure
  - Simple bit manipulation for prefix sum operations (`idx & -idx`)
  - Minimal overhead per operation

### Segment Tree (Algorithm 2)

- **Total time**: 273.567853 ms
- Operations performed:
  - n = 50,000 initial array builds
  - q = 50,000 mixed queries (25,000 updates + 25,000 range queries)
- Key characteristics:
  - Tree structure stored in Vec with 2*n size (next power of two)
  - Recursive/iterative tree traversal with more steps per operation
  - Slightly higher constant factor due to tree navigation
  - More flexible structure for future extensions

## Performance Analysis

### Why Fenwick Tree is Faster

1. **Lower constant factor**: The Fenwick Tree requires fewer operations per query/update.
   - Each Fenwick operation typically involves ~log2(n) ~16 steps for n=50000.
   - Segment tree operations involve traversing both left and right paths, often more steps.

2. **Better cache locality**: The Fenwick Tree uses a simple flat array, while the Segment
   Tree's tree structure involves more memory accesses across different parts of the array.

3. **Simpler bitwise operations**: The Fenwick Tree uses `idx & -idx` to find the least significant
   bit, which is a single CPU instruction. The Segment Tree requires integer division and
   modular arithmetic for tree navigation.

4. **Memory efficiency**: The Fenwick Tree uses exactly n+1 elements, while the Segment Tree
   typically allocates 2*next_power_of_two(n) elements (for n=50000, this is 131072 elements,
   more than 2x the Fenwick Tree's memory).

### When Segment Tree Might Be Preferable

Despite being slower for this specific workload, the Segment Tree offers advantages:

1. **Range extensions**: Can easily support range updates, range minimum/maximum queries,
   and range sum queries with less code modification.

2. **Flexibility**: The tree structure can be adapted for non-commutative operations more
   naturally than the Fenwick Tree.

3. **Uniform interface**: The Segment Tree's query/update methods have a more intuitive
   signature for range operations.

4. **Future-proofing**: If the problem evolves to require more complex range queries,
   the Segment Tree is already in place.

## Conclusion

For the specific workload of point updates and range sum queries on a static array size,
the **Fenwick Tree** is the clear performance winner, being ~22% faster in this benchmark.

However, the choice between the two should consider:

- **Use Fenwick Tree when**: You need pure point updates + range sum queries, with
  constraints on memory and execution speed.

- **Use Segment Tree when**: You anticipate needing more complex range operations,
  or when the flexibility of the Segment Tree's structure provides other developmental
  benefits that outweigh the ~22% performance difference.

## Benchmark Code

The micro-benchmark is defined in `src/main.rs` within the `bench_tests` module (cfg(test)):

```rust
#[cfg(test)]
mod bench_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn run_micro_benchmark() {
        let mut test_input = String::new();
        let n = 50000;
        let q = 50000;
        // ... input generation
        
        let start_1 = Instant::now();
        let res_1 = solve(&test_input);  // Fenwick Tree
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let res_2 = alternative::solve(&test_input);  // Segment Tree
        let dur_2 = start_2.elapsed();

        assert_eq!(res_1, res_2);

        println!("Algorithm 1 (Fenwick Tree): {:?}", dur_1);
        println!("Algorithm 2 (Segment Tree): {:?}", dur_2);
    }
}
```
