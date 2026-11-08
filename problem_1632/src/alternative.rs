#[allow(dead_code)]
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
                movies.push((b, a)); // sort by end time, then start time
            }
        }
    }

    movies.sort_unstable();

    // Sorted vector of member end times, initialized with k zeros
    let mut members: Vec<i32> = vec![0; k];
    let mut count = 0;

    for &(end, start) in &movies {
        // Find the rightmost member whose end time <= start
        let idx = members.partition_point(|&x| x <= start);
        if idx > 0 {
            // The optimal member is at idx - 1 (largest end time <= start)
            let chosen = idx - 1;
            // Shift elements to maintain sorted order after updating member's end time
            members.remove(chosen);
            let insert_idx = members.partition_point(|&x| x <= end);
            members.insert(insert_idx, end);
            count += 1;
        }
    }

    format!("{}\n", count)
}
