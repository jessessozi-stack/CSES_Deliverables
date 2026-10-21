mod alternative;

use std::io::{self, Read};

pub fn solve(input: &str) -> String {
    let s = input.trim();
    if s.is_empty() {
        return String::from("0\n");
    }

    let mut counts = [0usize; 26];
    for b in s.bytes() {
        if (b'a'..=b'z').contains(&b) {
            counts[(b - b'a') as usize] += 1;
        }
    }

    let mut results = Vec::new();
    let mut current = String::with_capacity(s.len());
    backtrack(s.len(), &mut current, &mut counts, &mut results);

    let mut output = format!("{}\n", results.len());
    for res in results {
        output.push_str(&res);
        output.push('\n');
    }
    output
}

fn backtrack(len: usize, current: &mut String, counts: &mut [usize; 26], results: &mut Vec<String>) {
    if current.len() == len {
        results.push(current.clone());
        return;
    }

    for i in 0..26 {
        if counts[i] > 0 {
            counts[i] -= 1;
            current.push((b'a' + i as u8) as char);
            backtrack(len, current, counts, results);
            current.pop();
            counts[i] += 1;
        }
    }
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
        let test_input = "aabacde\n";
        println!("\n--- MICRO-BENCHMARK COMPARISON ---");
        println!("Test Input: {}", test_input.trim());

        // 1. Benchmark Algorithm 1: Frequency Map Backtracking
        let start_1 = Instant::now();
        let result_1 = solve(test_input);
        let duration_1 = start_1.elapsed();

        // 2. Benchmark Algorithm 2: BTreeSet Permutation Swapping
        let start_2 = Instant::now();
        let result_2 = alternative::solve(test_input);
        let duration_2 = start_2.elapsed();

        // Ensure both produce identical outputs
        assert_eq!(result_1, result_2, "Both algorithms must produce identical results!");

        println!("Algorithm 1 (Frequency Map Backtracking): {:?}", duration_1);
        println!("Algorithm 2 (BTreeSet Permutation Swapping): {:?}", duration_2);
        println!("------------------------------------\n");
    }
}
