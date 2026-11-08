use std::collections::BTreeMap;
use std::ops::Bound;

#[allow(dead_code)]
pub fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();
    let n: usize = match iter.next() {
        Some(v) => v.parse().unwrap_or(0),
        None => return String::from("0\n"),
    };

    let mut map = BTreeMap::new();
    let mut total_towers = 0;

    for _ in 0..n {
        let cube: i32 = match iter.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };

        // Find the smallest tower top strictly greater than cube
        if let Some((&top, &count)) = map.range((Bound::Excluded(cube), Bound::Unbounded)).next() {
            if count == 1 {
                map.remove(&top);
            } else {
                map.insert(top, count - 1);
            }
            *map.entry(cube).or_insert(0) += 1;
        } else {
            *map.entry(cube).or_insert(0) += 1;
            total_towers += 1;
        }
    }

    format!("{}\n", total_towers)
}
