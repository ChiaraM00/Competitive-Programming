use std::io;
use std::collections::BTreeMap;
 
struct BinarySegmentTree {
    tree: Vec<(i64,i64)>,
    size: usize,
}
 
impl BinarySegmentTree {
    fn new(values: &[(i64, i64)]) -> Self {
 
        let mut size = 1;
        while size < values.len() {
            size *= 2;
        }
        let mut tree = vec![(0,0); 2 * size];
        
        for i in size..(values.len() + size) {
            tree[i] = values[i - size];
        }
        
		for i in (1..size).rev() {
			if tree[2 * i].1 >= tree[2 * i + 1].1 {
				tree[i] = tree[2 * i];
			} else {
				tree[i] = tree[2 * i + 1];
			}
		}
        BinarySegmentTree { tree, size }
    }
 
       fn query(&self, q: (i64, i64), saved_values: &mut BTreeMap<(i64, i64), (i64, i64)>,
       t: i64) -> usize {
              if q.0 < self.tree[self.size].0 {
                     return 0;
              }
              if q.0 > self.tree[1].1 {
                     //println!("per dopo");
                     saved_values.insert((q.0, t), (q.0, q.1));
                     return 0;
              }
              self.query_helper(q, 1, 0, self.size - 1, saved_values, t)
           }
 
 
    fn query_helper(&self, q: (i64,i64), idx: usize, segment_left: usize,
              segment_right: usize,  saved_values: &mut BTreeMap<(i64,i64), (i64, i64)>,
              t: i64)
              -> usize {
              
		let a = self.tree[idx].0;
		let b = self.tree[idx].1;
		//println!("({}, {})", a,b);
		
        if segment_left  == segment_right {
                     if q.0 > b {
                            self.query_helper(q, idx+1, segment_right + 1, segment_right + 1, saved_values, t)
                     } else if q.0>=a {
			       //println!("({}, {})", self.tree[idx].0, self.tree[idx].1);
				idx
			} else {
				//println!("per dopo");
                            saved_values.insert((q.0,t), (q.0, q.1));
				0
			}
        } 
        else {
                     
			let mid = (segment_left + segment_right) / 2;
			if q.0<=b {
				self.query_helper(q, 2 * idx, segment_left, mid, saved_values,t)
			} else {
			       self.query_helper(q, idx+1, segment_right + 1, 2 * segment_right - segment_left + 1, saved_values,t)
			}
        }
    }
    
    fn update(&mut self, idx: usize, value: i64) {
       let mut node = idx;
        self.tree[node].1 += value;
        while node> 1 {
            node /= 2;
            if self.tree[2 * node].1 >= self.tree[2 * node + 1].1 {
				self.tree[node] = self.tree[2 * node];
			} else {
				self.tree[node] = self.tree[2 * node + 1];
			}
        }
    }
    
    
    fn print_leaves(&self) {
        for i in 0..(2 * self.size) {
            print!("({}, {}) ", self.tree[i].0, self.tree[i].1);
        }
        println!();
    }
    
    
    
}
 
fn main() {
 
    let mut my_string = String::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let my_string: Vec<usize> = my_string.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let f = my_string[0];
	let m = my_string[1];
 
	let mut all_frogs: Vec<(i64,i64)> = Vec::new();
	for _ in 0..f {
        let mut _frog = String::new();
        io::stdin().read_line(&mut _frog).unwrap();
        let _frog: Vec<i64> = _frog.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        all_frogs.push((_frog[0], _frog[0] + _frog[1]));
       }
	
	let mut queries: Vec<(i64,i64)> = Vec::new();
	for _ in 0..m {
        let mut _mosq = String::new();
        io::stdin().read_line(&mut _mosq).unwrap();
        let _mosq: Vec<i64> = _mosq.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        queries.push((_mosq[0], _mosq[1]));
    }
	
    let mut saved_values: BTreeMap<(i64,i64), (i64, i64)> = BTreeMap::new();
    
    let mut indices: Vec<usize> = (0..all_frogs.len()).collect();
 
    indices.sort_by_key(|&i| all_frogs[i].0);
       
    let mut inverse_indices = vec![0; indices.len()];
    for (new_index, &old_index) in indices.iter().enumerate() {
           inverse_indices[old_index] = new_index;
    }
    
    let sorted_values: Vec<(i64, i64)> = indices.iter().map(|&i| all_frogs[i]).collect();
    
    
    let mut mosq_count = vec![0; sorted_values.len()];
    
    let mut seg_tree = BinarySegmentTree::new(&sorted_values);
    //seg_tree.print_leaves();
    
    let mut t = 0;
    for q in queries {
       //println!("{}, peso {}", q.0, q.1);
       let frog = seg_tree.query(q, &mut saved_values, t);
       if frog != 0 {
              //println!("{}", frog);
              mosq_count[frog - seg_tree.tree.len()/2] += 1;
              
              seg_tree.update(frog, q.1);
              //seg_tree.print_leaves();
              
              let mut keys_to_remove = Vec::new();
              for (&key, &saved_q) in saved_values.range((seg_tree.tree[frog].0,0) ..) {
                  if saved_q.0 <= seg_tree.tree[frog].1 {
                     mosq_count[frog - seg_tree.tree.len()/2] += 1;
                      seg_tree.update(frog, saved_q.1);
                      // seg_tree.print_leaves();
              
                      keys_to_remove.push(key.clone());
                  } else {
                      break;
                  }
              }
              
              for key in keys_to_remove {
                  saved_values.remove(&key);
              }
              
       }
       t+=1;
    }
    
    for i in inverse_indices{
       println!("{} {}", mosq_count[i], seg_tree.tree[i + seg_tree.tree.len()/2].1 - seg_tree.tree[i + seg_tree.tree.len()/2].0);
    }
 
 
}
 
 