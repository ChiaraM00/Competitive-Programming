use std::collections::HashMap;
use std::io;

struct BinarySegmentTree {
    tree: Vec<i32>,
    size: usize,
}

impl BinarySegmentTree {
    fn new(values: &[i32]) -> Self {
        let mut size = 1;
        while size < values.len() {
            size *= 2;
        }
        let mut tree = vec![0; 2 * size];
        BinarySegmentTree { tree, size }
    }

    fn range_query(&self, right: usize) -> i32 {
        self.range_query_helper(right, 1, 0, self.size - 1)
    }

    fn range_query_helper(&self, right: usize, idx: usize, segment_left: usize, segment_right: usize) -> i32 {
        if 0 <= segment_left && segment_right <= right {
            self.tree[idx]
        } 
        else if right < segment_left || segment_right < 0 {
            0
        } 
        else {
            let mid = (segment_left + segment_right) / 2;
            let left_sum = self.range_query_helper(right, 2 * idx, segment_left, mid);
            let right_sum = self.range_query_helper(right, 2 * idx + 1, mid + 1, segment_right);
            left_sum + right_sum
        }
    }
    
    fn update(&mut self, idx: usize) {
        let mut node = idx + self.size;
        self.tree[node] += 1;
        while node > 1 {
            node /= 2;
            self.tree[node] += 1;
        }
    }
    
    fn print_leaves(&self) {
        for i in 0..(2 * self.size) {
            print!("{} ", self.tree[i]);
        }
        println!();
    }
}

fn main() {

    let mut my_string = String::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let my_string: Vec<usize> = my_string.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let n = my_string[0];
	let m = my_string[1];

    let mut arr = String::new();
    io::stdin().read_line(&mut arr).unwrap();
    let arr: Vec<usize> = arr.trim().split(' ').map(|x| x.parse().unwrap()).collect();

    let mut queries_old: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..m {
        let mut _queries_old = String::new();
        io::stdin().read_line(&mut _queries_old).unwrap();
        let _queries_old: Vec<usize> = _queries_old.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        queries_old.push((_queries_old[0], _queries_old[1], _queries_old[2]));
    }

    let values = vec![0;n+1];
    let mut a = 0;

    let mut queries = Vec::new();
    for &(a, b, c) in &queries_old {
        queries.push((a, c));
        queries.push((b+1, c));
    }
    
    let mut sorted_queries: Vec<(usize, usize)> = queries.to_vec();
    sorted_queries.sort_by_key(|&tuple| tuple.0);
    
    let mut seg_tree = BinarySegmentTree::new(&values);
    let mut results: HashMap<(usize, usize), i32> = HashMap::new();
        
    for q in sorted_queries {
        while q.0 as usize > a {
            seg_tree.update(arr[a as usize] as usize);
            a += 1;
            //seg_tree.print_leaves();
        }
        results.insert(q, seg_tree.range_query(q.1));
    }
    
    
    let mut result_values: Vec<i32> = Vec::new();
    for q in &queries {
        result_values.push(results[&q]);
    }

    let mut diff_values: Vec<i32> = Vec::new();
    for i in (0..result_values.len()).step_by(2) {
        let diff = -result_values[i] + result_values[i + 1];
        diff_values.push(diff);
    }

    for diff in &diff_values {
        println!("{}", diff);
    }

}