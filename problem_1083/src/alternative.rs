pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let n_line = match lines.next() {
        Some(l) => l.trim(),
        None => return String::from(""),
    };
    if n_line.is_empty() {
        return String::from("");
    }
    let n: usize = n_line.parse().unwrap();

    let second_line = match lines.next() {
        Some(l) => l,
        None => return String::from(""),
    };

    // Collect numbers into a dynamic vector
    let mut nums: Vec<usize> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Sort the dynamic vector to find the missing gap
    nums.sort_unstable();

    let mut missing = n;
    for (i, &val) in nums.iter().enumerate() {
        if val != i + 1 {
            missing = i + 1;
            break;
        }
    }

    format!("{}\n", missing)
}
