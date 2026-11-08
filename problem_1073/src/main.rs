mod alternative;

use std::io::{self, Read};

pub fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();
    let n: usize = match iter.next() {
        Some(v) => v.parse().unwrap_or(0),
        None => return String::from("0\n"),
    };

    let mut towers = Vec::with_capacity(n);
    for _ in 0..n {
        let cube: i32 = match iter.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };

        let idx = towers.partition_point(|&x| x <= cube);
        if idx < towers.len() {
            towers[idx] = cube;
        } else {
            towers.push(cube);
        }
    }

    format!("{}\n", towers.len())
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
        test_input.push_str(&format!("{}\n", n));

        // Generate synthetic test data with repeating cube sizes
        for i in 1..=n {
            test_input.push_str(&format!("{} ", (i * 37) % 100000));
        }
        test_input.push('\n');

        println!("\n--- MICRO-BENCHMARK: TOWERS ---");

        let start_1 = Instant::now();
        let res_1 = solve(&test_input);
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let res_2 = alternative::solve(&test_input);
        let dur_2 = start_2.elapsed();

        assert_eq!(res_1, res_2);

        println!("Algorithm 1 (Sorted Vector + Binary Search): {:?}", dur_1);
        println!("Algorithm 2 (BTreeMap Frequency Map): {:?}", dur_2);
        println!("---------------------------------------\n");
    }
}
