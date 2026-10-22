mod alternative;

use std::io::{self, Read};
use std::collections::VecDeque;

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let n_line = match lines.next() {
        Some(l) => l.trim(),
        None => return String::from("0\n"),
    };
    if n_line.is_empty() {
        return String::from("0\n");
    }
    let n: usize = match n_line.parse() {
        Ok(val) => val,
        Err(_) => return String::from("0\n"),
    };

    if n <= 1 {
        return String::from("0\n");
    }

    let mut adj = vec![vec![]; n + 1];
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        if let (Some(u_str), Some(v_str)) = (parts.next(), parts.next()) {
            if let (Ok(u), Ok(v)) = (u_str.parse::<usize>(), v_str.parse::<usize>()) {
                adj[u].push(v);
                adj[v].push(u);
            }
        }
    }

    fn farthest_node(start: usize, n: usize, adj: &Vec<Vec<usize>>) -> (usize, usize) {
        let mut dist = vec![-1; n + 1];
        let mut queue = VecDeque::new();

        dist[start] = 0;
        queue.push_back(start);

        let mut farthest = start;
        let mut max_dist = 0;

        while let Some(u) = queue.pop_front() {
            if dist[u] > max_dist {
                max_dist = dist[u];
                farthest = u;
            }

            for &v in &adj[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    queue.push_back(v);
                }
            }
        }

        (farthest, max_dist as usize)
    }

    let (node_a, _) = farthest_node(1, n, &adj);
    let (_, diameter) = farthest_node(node_a, n, &adj);

    format!("{}\n", diameter)
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
        let n = 20000;
        test_input.push_str(&format!("{}\n", n));
        
        // Construct a balanced binary tree (node i connects to parent i / 2)
        // This keeps max depth around log2(20000) ≈ 15 levels, preventing stack overflow.
        for i in 2..=n {
            test_input.push_str(&format!("{} {}\n", i / 2, i));
        }

        println!("\n--- MICRO-BENCHMARK: TREE DIAMETER ---");

        let start_1 = Instant::now();
        let res_1 = solve(&test_input);
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let res_2 = alternative::solve(&test_input);
        let dur_2 = start_2.elapsed();

        assert_eq!(res_1, res_2);

        println!("Algorithm 1 (Two-Pass BFS): {:?}", dur_1);
        println!("Algorithm 2 (Tree DP): {:?}", dur_2);
        println!("---------------------------------------\n");
    }
}      
