mod alternative;

use std::io::{self, Read};

pub fn solve(input: &str) -> String {
    let mut reserved_rows = [0u8; 8];
    let mut row_idx = 0;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.len() == 8 && trimmed.chars().all(|c| c == '.' || c == '*') {
            let mut mask = 0u8;
            for (col_idx, ch) in trimmed.chars().enumerate() {
                if ch == '*' {
                    mask |= 1u8 << col_idx;
                }
            }
            reserved_rows[row_idx] = mask;
            row_idx += 1;
            if row_idx == 8 {
                break;
            }
        }
    }

    if row_idx < 8 {
        return String::from("0\n");
    }

    fn dfs(
        r: usize,
        cols: u8,
        diag1: u16,
        diag2: u16,
        reserved_rows: &[u8; 8],
    ) -> u64 {
        if r == 8 {
            return 1;
        }

        let mut total = 0;
        let blocked_base = cols | reserved_rows[r];

        for c in 0..8 {
            let col_bit = 1u8 << c;
            if (blocked_base & col_bit) != 0 {
                continue;
            }

            let d1_bit = 1u16 << (r + c);
            let d2_bit = 1u16 << (r + 7 - c);
            if (diag1 & d1_bit) == 0 && (diag2 & d2_bit) == 0 {
                total += dfs(
                    r + 1,
                    cols | col_bit,
                    diag1 | d1_bit,
                    diag2 | d2_bit,
                    reserved_rows,
                );
            }
        }

        total
    }

    let ans = dfs(0, 0, 0, 0, &reserved_rows);
    format!("{}\n", ans)
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        print!("{}", solve(&input));
    }
}

#[cfg(test)]
mod bench_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn run_micro_benchmark() {
        let sample_input = "\
........
........
..*.....
........
........
.....**...
...*....
........
";
        // Clean 8x8 sample input
        let clean_input = "\
........
........
..*.....
........
........
.....**A
...*....
........
"
        .replace('A', ".");

        let _ = sample_input;
        let iterations = 10_000;

        println!("\n--- MICRO-BENCHMARK: CHESSBOARD AND QUEENS ---");

        let start_1 = Instant::now();
        let mut res_1 = String::new();
        for _ in 0..iterations {
            res_1 = solve(&clean_input);
        }
        let dur_1 = start_1.elapsed();

        let start_2 = Instant::now();
        let mut res_2 = String::new();
        for _ in 0..iterations {
            res_2 = alternative::solve(&clean_input);
        }
        let dur_2 = start_2.elapsed();

        assert_eq!(res_1, res_2);

        println!("Algorithm 1 (Bitmask Backtracking x{}): {:?}", iterations, dur_1);
        println!("Algorithm 2 (Array Backtracking x{}): {:?}", iterations, dur_2);
        println!("Result: {}", res_1.trim());
        println!("----------------------------------------------\n");
    }
}
