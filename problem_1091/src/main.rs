mod alternative;

use std::collections::BTreeMap;
use std::io::{self, Read};

pub fn solve(input: &str) -> String {
    let mut input_iter = input.split_whitespace();
    let n: usize = match input_iter.next() {
        Some(v) => v.parse().unwrap_or(0),
        None => return String::new(),
    };
    let m: usize = input_iter.next().unwrap().parse().unwrap();

    let mut map = BTreeMap::new();
    for _ in 0..n {
        let price: i32 = input_iter.next().unwrap().parse().unwrap();
        *map.entry(price).or_insert(0) += 1;
    }

    let mut output = String::new();
    for _ in 0..m {
        let max_price: i32 = input_iter.next().unwrap().parse().unwrap();
        if let Some((&price, &count)) = map.range(..=max_price).next_back() {
            output.push_str(&format!("{}\n", price));
            if count == 1 {
                map.remove(&price);
            } else {
                map.insert(price, count - 1);
            }
        } else {
            output.push_str("-1\n");
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
        let m = 50000;
        test_input.push_str(&format!("{} {}\n", n, m));

        for i in 1..=n {
            test_input.push_str(&format!("{} ", i * 2));
        }
        test_input.push('\n');

        for i in 1..=m {
            test_input.push_str(&format!("{} ", i * 2 + 1));
        }
        test_input.push('\n');

        println!("\n--- MICRO-BENCHMARK: CONCERT TICKETS ---");

        let start_1 = Instant::now();
        let res_1 = solve(&test_input);
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let res_2 = alternative::solve(&test_input);
        let dur_2 = start_2.elapsed();

        assert_eq!(res_1, res_2);

        println!("Algorithm 1 (BTreeMap Frequency): {:?}", dur_1);
        println!("Algorithm 2 (Indexed BTreeSet): {:?}", dur_2);
        println!("---------------------------------------\n");
    }
}
