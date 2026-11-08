use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn solve(input: &str) -> String {
    let mut input_iter = input.split_whitespace();
    let n: usize = match input_iter.next() {
        Some(v) => v.parse().unwrap_or(0),
        None => return String::new(),
    };
    let m: usize = input_iter.next().unwrap().parse().unwrap();

    let mut set = BTreeSet::new();
    for i in 0..n {
        let price: i32 = input_iter.next().unwrap().parse().unwrap();
        set.insert((price, i));
    }

    let mut output = String::new();
    for _ in 0..m {
        let max_price: i32 = input_iter.next().unwrap().parse().unwrap();
        // Query greatest element <= (max_price, usize::MAX)
        let query_key = (max_price, usize::MAX);
        if let Some(&ticket) = set.range(..=query_key).next_back() {
            output.push_str(&format!("{}\n", ticket.0));
            set.remove(&ticket);
        } else {
            output.push_str("-1\n");
        }
    }
    output
}
