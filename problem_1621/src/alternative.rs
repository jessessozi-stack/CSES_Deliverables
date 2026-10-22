#[allow(dead_code)]
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

    let mut nums: Vec<i64> = second_line
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    if nums.is_empty() {
        return String::from("0\n");
    }

    nums.sort_unstable();
    let mut distinct_count = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            distinct_count += 1;
        }
    }

    format!("{}\n", distinct_count)
}
