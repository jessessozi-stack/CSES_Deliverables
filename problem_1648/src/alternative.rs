struct SegmentTree {
    tree: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self {
            tree: vec![0; 2 * n],
            n,
        }
    }

    fn build(&mut self, arr: &[i64]) {
        for (i, &val) in arr.iter().enumerate() {
            self.tree[self.n + i] = val;
        }
        for i in (1..self.n).rev() {
            self.tree[i] = self.tree[2 * i] + self.tree[2 * i + 1];
        }
    }

    fn update(&mut self, mut idx: usize, val: i64) {
        idx += self.n;
        self.tree[idx] = val;
        idx /= 2;
        while idx > 0 {
            self.tree[idx] = self.tree[2 * idx] + self.tree[2 * idx + 1];
            idx /= 2;
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> i64 {
        if l > r {
            return 0;
        }
        l += self.n;
        r += self.n;
        let mut sum = 0;
        while l <= r {
            if l % 2 == 1 {
                sum += self.tree[l];
                l += 1;
            }
            if r % 2 == 0 {
                sum += self.tree[r];
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        sum
    }
}

#[allow(dead_code)]
pub fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();
    let n: usize = match iter.next() {
        Some(v) => v.parse::<usize>().unwrap(),
        None => return String::new(),
    };
    let q: usize = iter.next().unwrap().parse::<usize>().unwrap();

    let mut arr = vec![0i64; n];
    for i in 0..n {
        arr[i] = iter.next().unwrap().parse::<i64>().unwrap();
    }

    let mut seg_tree = SegmentTree::new(n);
    seg_tree.build(&arr);

    let mut output = String::new();
    for _ in 0..q {
        let t: i32 = iter.next().unwrap().parse::<i32>().unwrap();
        if t == 1 {
            let k: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let u: i64 = iter.next().unwrap().parse::<i64>().unwrap();
            seg_tree.update(k, u);
        } else {
            let a: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let b: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let ans = seg_tree.query(a, b);
            output.push_str(&format!("{}\n", ans));
        }
    }
    output
}
