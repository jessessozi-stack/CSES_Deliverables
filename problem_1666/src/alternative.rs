#[allow(dead_code)]
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
    
    let mut parent: Vec<usize> = (0..=n).collect();

    fn find(i: usize, parent: &mut Vec<usize>) -> usize {
        let mut root = i;
        while root != parent[root] {
            root = parent[root];
        }
        let mut curr = i;
        while curr != root {
            let nxt = parent[curr];
            parent[curr] = root;
            curr = nxt;
        }
        root
    }

    fn union(i: usize, j: usize, parent: &mut Vec<usize>) -> bool {
        let root_i = find(i, parent);
        let root_j = find(j, parent);
        if root_i != root_j {
            parent[root_i] = root_j;
            true
        } else {
            false
        }
    }

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        if let (Some(u_str), Some(v_str)) = (parts.next(), parts.next()) {
            if let (Ok(u), Ok(v)) = (u_str.parse::<usize>(), v_str.parse::<usize>()) {
                union(u, v, &mut parent);
            }
        }
    }

    let mut components = Vec::new();
    let base_root = find(1, &mut parent);
    for i in 2..=n {
        let root = find(i, &mut parent);
        if root != base_root {
            parent[root] = base_root;
            components.push((1, i));
        }
    }

    let mut output = format!("{}\n", components.len());
    for (u, v) in components {
        output.push_str(&format!("{} {}\n", u, v));
    }
    output
}
