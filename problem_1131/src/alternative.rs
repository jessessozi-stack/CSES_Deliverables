#[allow(dead_code)]
pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let n_line = match lines.next() {
        Some(l) => l.trim(),
        None => return String::from("0\n"),
    };
    if n_line.is_empty() {
        return String::from("0\n");
    }
    let n: usize = match n_line.parse() {
        Ok(val) => val,
        Err(_) => return String::from("0\n"),
    };

    if n <= 1 {
        return String::from("0\n");
    }

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

    let mut max_diameter = 0;

    fn dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>, max_diameter: &mut usize) -> usize {
        let mut max1 = 0;
        let mut max2 = 0;

        for &v in &adj[u] {
            if v != p {
                let depth = dfs(v, u, adj, max_diameter);
                if depth > max1 {
                    max2 = max1;
                    max1 = depth;
                } else if depth > max2 {
                    max2 = depth;
                }
            }
        }

        *max_diameter = (*max_diameter).max(max1 + max2);
        max1 + 1
    }

    dfs(1, 0, &adj, &mut max_diameter);
    format!("{}\n", max_diameter)
}
