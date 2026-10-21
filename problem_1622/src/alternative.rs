use std::collections::BTreeSet;

pub fn solve(input: &str) -> String {
    let s = input.trim();
    if s.is_empty() {
        return String::from("0\n");
    }

    let mut chars: Vec<char> = s.chars().collect();
    let mut results = BTreeSet::new();
    permute(&mut chars, 0, &mut results);

    let mut output = format!("{}\n", results.len());
    for res in results {
        output.push_str(&res);
        output.push('\n');
    }
    output
}

fn permute(chars: &mut [char], index: usize, results: &mut BTreeSet<String>) {
    if index == chars.len() {
        results.insert(chars.iter().collect());
        return;
    }
    for i in index..chars.len() {
        chars.swap(index, i);
        permute(chars, index + 1, results);
        chars.swap(index, i);
    }
}
