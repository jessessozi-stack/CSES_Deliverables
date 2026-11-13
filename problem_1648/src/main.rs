mod alternative;

use std::io::{self, Read};

struct FenwickTree {
    bit: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            bit: vec![0; n + 1],
            n,
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.bit[idx] += delta;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut sum = 0;
        while idx > 0 {
            sum += self.bit[idx];
            idx -= idx & idx.wrapping_neg();
        }
        sum
    }

    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.query(r) - self.query(l - 1)
    }
}

pub fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();
    let n: usize = match iter.next() {
        Some(v) => v.parse().unwrap(),
        None => return String::new(),
    };
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut arr = vec![0i64; n + 1];
    let mut bit = FenwickTree::new(n);

    for i in 1..=n {
        let val: i64 = iter.next().unwrap().parse().unwrap();
        arr[i] = val;
        bit.update(i, val);
    }

    let mut output = String::new();
    for _ in 0..q {
        let t: i32 = iter.next().unwrap().parse().unwrap();
        if t == 1 {
            let k: usize = iter.next().unwrap().parse().unwrap();
            let u: i64 = iter.next().unwrap().parse().unwrap();
            let diff = u - arr[k];
            arr[k] = u;
            bit.update(k, diff);
        } else {
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: usize = iter.next().unwrap().parse().unwrap();
            let ans = bit.range_query(a, b);
            output.push_str(&format!("{}\n", ans));
        }
    }
    output
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        print!("{}", solve(&input));
    }
}

// --- Micro-Benchmark Inline Test ---
#[cfg(test)]
mod bench_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn run_micro_benchmark() {
        let mut test_input = String::new();
        let n = 50000;
        let q = 50000;
        test_input.push_str(&format!("{} {}\n", n, q));

        for i in 1..=n {
            test_input.push_str(&format!("{} ", i));
        }
        test_input.push('\n');

        for i in 1..=q {
            if i % 2 == 0 {
                let k = (i % n) + 1;
                let u = i as i64;
                test_input.push_str(&format!("1 {} {}\n", k, u));
            } else {
                let a = ((i * 3) % n) + 1;
                let b = a + (i % (n - a + 1));
                test_input.push_str(&format!("2 {} {}\n", a, b));
            }
        }

        println!("\n--- MICRO-BENCHMARK: DYNAMIC RANGE SUM QUERIES ---");

        let start_1 = Instant::now();
        let res_1 = solve(&test_input);
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let res_2 = alternative::solve(&test_input);
        let dur_2 = start_2.elapsed();

        assert_eq!(res_1, res_2);

        println!("Algorithm 1 (Fenwick Tree): {:?}", dur_1);
        println!("Algorithm 2 (Segment Tree): {:?}", dur_2);
        println!("-----------------------------------------------\n");
    }
}
