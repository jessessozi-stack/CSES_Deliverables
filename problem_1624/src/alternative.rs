#[allow(dead_code)]
pub fn solve(input: &str) -> String {
    let mut reserved = [[false; 8]; 8];
    let mut row_idx = 0;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.len() == 8 && trimmed.chars().all(|c| c == '.' || c == '*') {
            for (col_idx, ch) in trimmed.chars().enumerate() {
                if ch == '*' {
                    reserved[row_idx][col_idx] = true;
                }
            }
            row_idx += 1;
            if row_idx == 8 {
                break;
            }
        }
    }

    if row_idx < 8 {
        return String::from("0\n");
    }

    let mut col_used = [false; 8];
    let mut diag1_used = [false; 15]; // r + c in 0..=14
    let mut diag2_used = [false; 15]; // r + 7 - c in 0..=14
    let mut count: u64 = 0;

    fn backtrack(
        r: usize,
        reserved: &[[bool; 8]; 8],
        col_used: &mut [bool; 8],
        diag1_used: &mut [bool; 15],
        diag2_used: &mut [bool; 15],
        count: &mut u64,
    ) {
        if r == 8 {
            *count += 1;
            return;
        }

        for c in 0..8 {
            let d1 = r + c;
            let d2 = r + 7 - c;
            if reserved[r][c] || col_used[c] || diag1_used[d1] || diag2_used[d2] {
                continue;
            }

            col_used[c] = true;
            diag1_used[d1] = true;
            diag2_used[d2] = true;

            backtrack(r + 1, reserved, col_used, diag1_used, diag2_used, count);

            col_used[c] = false;
            diag1_used[d1] = false;
            diag2_used[d2] = false;
        }
    }

    backtrack(
        0,
        &reserved,
        &mut col_used,
        &mut diag1_used,
        &mut diag2_used,
        &mut count,
    );

    format!("{}\n", count)
}
