mod alternative;

use std::io::{self, Read};

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let first_line = match lines.next() {
        Some(l) => l.trim(),
        None => return String::from("0\n"),
    };
    if first_line.is_empty() {
        return String::from("0\n");
    }

    let mut header = first_line.split_whitespace();
    let n: usize = header.next().unwrap().parse().unwrap();

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

    let mut visited = vec![false; n + 1];
    let mut leaders = Vec::new();

    fn dfs(u: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
        visited[u] = true;
        for &v in &adj[u] {
            if !visited[v] {
                dfs(v, adj, visited);
            }
        }
    }

    for i in 1..=n {
        if !visited[i] {
            leaders.push(i);
            dfs(i, &adj, &mut visited);
        }
    }

    let mut output = format!("{}\n", if leaders.is_empty() { 0 } else { leaders.len() - 1 });
    for i in 0..leaders.len().saturating_sub(1) {
        output.push_str(&format!("{} {}\n", leaders[i], leaders[i + 1]));
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
        let n = 20000;
        test_input.push_str(&format!("{} 0\n", n));

        println!("\n--- MICRO-BENCHMARK: BUILDING ROADS ---");

        let start_1 = Instant::now();
        let res_1 = solve(&test_input);
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let res_2 = alternative::solve(&test_input);
        let dur_2 = start_2.elapsed();

        // Verify both solutions produce the correct number of new roads (n - 1)
        let count_1: usize = res_1.lines().next().unwrap_or("0").parse().unwrap_or(0);
        let count_2: usize = res_2.lines().next().unwrap_or("0").parse().unwrap_or(0);
        
        assert_eq!(count_1, n - 1);
        assert_eq!(count_2, n - 1);

        println!("Algorithm 1 (DFS Components): {:?}", dur_1);
        println!("Algorithm 2 (DSU Union-Find): {:?}", dur_2);
        println!("---------------------------------------\n");
    }
}
