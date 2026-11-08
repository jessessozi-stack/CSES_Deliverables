mod alternative;

use std::collections::BTreeSet;
use std::io::{self, Read};

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let first = match lines.next() {
        Some(l) => l.trim(),
        None => return String::from("0\n"),
    };
    if first.is_empty() {
        return String::from("0\n");
    }
    let mut header = first.split_whitespace();
    let n: usize = header.next().unwrap().parse().unwrap();
    let k: usize = header.next().unwrap().parse().unwrap();

    let mut movies = Vec::with_capacity(n);
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        if let (Some(a_str), Some(b_str)) = (parts.next(), parts.next()) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                movies.push((b, a));
            }
        }
    }

    movies.sort_unstable();

    let mut members = BTreeSet::new();
    for i in 0..k {
        members.insert((0, i));
    }

    let mut count = 0;
    let mut next_id = k;

    for &(end, start) in &movies {
        let query = (start, usize::MAX);
        if let Some(&target) = members.range(..=query).next_back() {
            members.remove(&target);
            members.insert((end, next_id));
            next_id += 1;
            count += 1;
        }
    }

    format!("{}\n", count)
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

    fn calculate_stats(durations: &mut [std::time::Duration]) -> (f64, f64, f64, f64, f64) {
        durations.sort();
        let n = durations.len();
        
        let min = durations[0].as_secs_f64() * 1000.0;
        let max = durations[n - 1].as_secs_f64() * 1000.0;
        let median = if n % 2 == 0 {
            (durations[n / 2 - 1].as_secs_f64() + durations[n / 2].as_secs_f64()) * 500.0
        } else {
            durations[n / 2].as_secs_f64() * 1000.0
        };

        let sum: f64 = durations.iter().map(|d| d.as_secs_f64() * 1000.0).sum();
        let mean = sum / n as f64;

        let variance: f64 = durations.iter()
            .map(|d| {
                let diff = (d.as_secs_f64() * 1000.0) - mean;
                diff * diff
            })
            .sum::<f64>() / n as f64;
        let std_dev = variance.sqrt();

        (min, max, mean, median, std_dev)
    }

    #[test]
    fn run_detailed_micro_benchmark() {
        let mut test_input = String::new();
        let n = 15000;
        let k = 100;
        test_input.push_str(&format!("{} {}\n", n, k));

        for i in 0..n {
            let start = i * 2;
            let end = start + 5;
            test_input.push_str(&format!("{} {}\n", start, end));
        }

        let iterations = 50;
        let mut times_1 = Vec::with_capacity(iterations);
        let mut times_2 = Vec::with_capacity(iterations);

        // Warm-up phase
        for _ in 0..5 {
            let _ = solve(&test_input);
            let _ = alternative::solve(&test_input);
        }

        // Measured execution phase
        for _ in 0..iterations {
            let start = Instant::now();
            let _ = solve(&test_input);
            times_1.push(start.elapsed());

            let start = Instant::now();
            let _ = alternative::solve(&test_input);
            times_2.push(start.elapsed());
        }

        let (min_1, max_1, mean_1, med_1, std_1) = calculate_stats(&mut times_1);
        let (min_2, max_2, mean_2, med_2, std_2) = calculate_stats(&mut times_2);

        println!("\n========================================================");
        println!(" DETAILED STATISTICAL MICRO-BENCHMARK: MOVIE FESTIVAL II");
        println!(" Workload: N = {}, K = {} over {} iterations", n, k, iterations);
        println!("========================================================");
        println!("Algorithm 1 (BTreeSet Multiset):");
        println!("  - Mean:   {:.3} ms", mean_1);
        println!("  - Median: {:.3} ms", med_1);
        println!("  - Min:    {:.3} ms", min_1);
        println!("  - Max:    {:.3} ms", max_1);
        println!("  - StdDev: +/- {:.3} ms", std_1);
        println!("--------------------------------------------------------");
        println!("Algorithm 2 (Sorted Vector Simulation):");
        println!("  - Mean:   {:.3} ms", mean_2);
        println!("  - Median: {:.3} ms", med_2);
        println!("  - Min:    {:.3} ms", min_2);
        println!("  - Max:    {:.3} ms", max_2);
        println!("  - StdDev: +/- {:.3} ms", std_2);
        println!("========================================================\n");
    }
}
