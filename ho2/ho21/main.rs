use std::io;

struct BinarySegmentTree {
    tree: Vec<(i64, bool)>,
    size: usize,
}

impl BinarySegmentTree {
    fn new(values: &Vec<i64>) -> Self {
        let mut size = 1;
        while size < values.len() {
            size *= 2;
        }
        let mut tree = vec![(0, true); 2 * size];

        for i in size..(values.len() + size) {
            tree[i].0 = values[i - size];
        }

        for i in (1..size).rev() {
            if tree[2 * i].0 >= tree[2 * i + 1].0 {
                tree[i] = tree[2 * i];
            } else {
                tree[i] = tree[2 * i + 1];
            }
        }
        BinarySegmentTree { tree, size }
    }

    fn query(&mut self, q: (i64, i64, i64, i64)) -> i64 {
        self.query_helper(q, 1, 0, self.size - 1)
    }

    fn query_helper(
        &mut self,
        q: (i64, i64, i64, i64),
        idx: usize,
        segment_left: usize,
        segment_right: usize,
    ) -> i64 {
        if self.tree[idx].1 == false {
            self.solve(idx);
        }

        if q.1 <= segment_left as i64 && q.2 >= segment_right as i64 {
            return self.tree[idx].0;
        }

        if segment_right < q.1 as usize || segment_left > q.2 as usize {
            return std::i64::MIN;
        }

        let mid = (segment_left + segment_right) / 2;
        std::cmp::max(
            self.query_helper(q, 2 * idx, segment_left, mid),
            self.query_helper(q, 2 * idx + 1, mid + 1, segment_right),
        )
    }

    fn update(&mut self, q: (i64, i64, i64, i64)) {
        self.update_helper(q, 1, 0, self.size - 1);
    }

    fn update_helper(
        &mut self,
        q: (i64, i64, i64, i64),
        idx: usize,
        segment_left: usize,
        segment_right: usize,
    ) {

        if q.1 <= segment_left as i64 && q.2 >= segment_right as i64 {
            if self.tree[idx].0 > q.3 {
                self.update_up(idx, q.3);
                self.tree[idx].1 = false;
            }
            return;
        }

        if segment_right < q.1 as usize || segment_left > q.2 as usize {
            return;
        }

        if self.tree[idx].1 == false {
            self.solve(idx);
        }
        let mid = (segment_left + segment_right) / 2;
        std::cmp::max(
            self.update_helper(q, 2 * idx, segment_left, mid),
            self.update_helper(q, 2 * idx + 1, mid + 1, segment_right),
        );
        return;
    }

    fn update_up(&mut self, idx: usize, value: i64) {
        let mut node = idx;
        self.tree[node].0 = value;
        while node > 1 {
            node /= 2;
            if self.tree[2 * node] >= self.tree[2 * node + 1] {
                self.tree[node] = self.tree[2 * node];
            } else {
                self.tree[node] = self.tree[2 * node + 1];
            }
        }
    }

    fn solve(&mut self, idx: usize) {
        self.tree[idx].1 = true;
        if idx < self.size {
            if self.tree[2 * idx].0 > self.tree[idx].0 {
                self.tree[2 * idx].1 = false;
                self.tree[2 * idx].0 = self.tree[idx].0;
            }
            if self.tree[2 * idx + 1].0 > self.tree[idx].0 {
                self.tree[2 * idx + 1].1 = false;
                self.tree[2 * idx + 1].0 = self.tree[idx].0;
            }
        }
    }

    fn print_leaves(&self) {
        for i in 0..(2 * self.size) {
            print!("{} ", self.tree[i].0);
        }
        println!();
    }
}

fn main() {
    let mut my_string = String::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let my_string: Vec<usize> = my_string
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let n = my_string[0];
    let m = my_string[1];

    let mut my_string = String::new();
    let mut nums: Vec<i64> = Vec::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let nums: Vec<i64> = my_string
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut queries: Vec<(i64, i64, i64, i64)> = Vec::new();
    for _ in 0..m {
        let mut _st = String::new();
        io::stdin().read_line(&mut _st).unwrap();
        let _st: Vec<i64> = _st.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        if _st[0] == 1 {
            queries.push((_st[0], _st[1] - 1, _st[2] - 1, 0));
        } else {
            queries.push((_st[0], _st[1] - 1, _st[2] - 1, _st[3]));
        }
    }

    let mut seg_tree = BinarySegmentTree::new(&nums);
    //seg_tree.print_leaves();

    for q in queries {
        if q.0 == 1 {
            let ris = seg_tree.query(q);
            println!("{}", ris);
        } else {
            seg_tree.update(q);
            //seg_tree.print_leaves();
        }
    }
}
