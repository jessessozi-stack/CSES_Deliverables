mod alternative;

use std::io::{self, Read};
use std::collections::BTreeSet;

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let _n = match lines.next() {
        Some(l) => l.trim(),
        None => return String::from("0\n"),
    };
    if _n.is_empty() {
        return String::from("0\n");
    }

    let second_line = match lines.next() {
        Some(l) => l,
        None => return String::from("0\n"),
    };

    let mut set = BTreeSet::new();
    for token in second_line.split_whitespace() {
        if let Ok(val) = token.parse::<i64>() {
            set.insert(val);
        }
    }

    format!("{}\n", set.len())
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
        test_input.push_str("200000\n");
        let numbers: Vec<String> = (0..200000).map(|i| (i % 50000).to_string()).collect();
        test_input.push_str(&numbers.join(" "));
        test_input.push('\n');

        println!("\n--- MICRO-BENCHMARK: DISTINCT NUMBERS ---");

        let start_1 = Instant::now();
        let res_1 = solve(&test_input);
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let res_2 = alternative::solve(&test_input);
        let dur_2 = start_2.elapsed();

        assert_eq!(res_1, res_2);

        println!("Algorithm 1 (BTreeSet): {:?}", dur_1);
        println!("Algorithm 2 (Sorting + Linear Scan): {:?}", dur_2);
        println!("-----------------------------------------\n");
    }
}
