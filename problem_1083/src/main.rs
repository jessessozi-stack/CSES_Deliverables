mod alternative;

use std::io::{self, Read};

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let n_line = match lines.next() {
        Some(l) => l.trim(),
        None => return String::from(""),
    };
    if n_link_empty(n_line) {
        return String::from("");
    }
    let n: usize = n_line.parse().unwrap();

    let second_line = match lines.next() {
        Some(l) => l,
        None => return String::from(""),
    };

    let mut sum: usize = 0;
    for token in second_line.split_whitespace() {
        if let Ok(val) = token.parse::<usize>() {
            sum += val;
        }
    }

    // Expected sum of numbers from 1 to n: n * (n + 1) / 2
    let total_sum = n * (n + 1) / 2;
    let missing = total_sum - sum;

    format!("{}\n", missing)
}

fn n_link_empty(s: &str) -> bool {
    s.is_empty()
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
        // Construct a realistic test case with n = 200,000 missing a number
        let mut test_input = String::new();
        test_input.push_str("200000\n");
        let mut numbers: Vec<String> = (1..=200000).map(|i| i.to_string()).collect();
        // Remove one number in the middle (e.g., 100005)
        numbers.remove(100004);
        test_input.push_str(&numbers.join(" "));
        test_input.push('\n');

        println!("\n--- MICRO-BENCHMARK: MISSING NUMBER ---");

        let start_1 = Instant::now();
        let res_1 = solve(&test_input);
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let res_2 = alternative::solve(&test_input);
        let dur_2 = start_2.elapsed();

        assert_eq!(res_1, res_2);

        println!("Algorithm 1 (Math Sum Formula): {:?}", dur_1);
        println!("Algorithm 2 (Dynamic Vector Sorting): {:?}", dur_2);
        println!("---------------------------------------\n");
    }
}
