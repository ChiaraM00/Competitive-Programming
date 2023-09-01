use rand::Rng;
use std::collections::BinaryHeap;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut result = Vec::new();
    let mut heap = BinaryHeap::new();
    for i in 0..nums.len() {
        while let Some(&(_max_value, index)) = heap.peek() {
            if index + k <= i {
                heap.pop();
            } else {
                break;
            }
        }
        heap.push((nums[i], i));
        if i >= k - 1 {
            if let Some(&(max_value, _)) = heap.peek() {
                result.push(max_value);
            }
        }
    }
    result
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::BTreeMap;
    let k = k as usize;
    let mut result = Vec::new();
    let mut bst: BTreeMap<i32, usize> = BTreeMap::new();
    for i in 0..nums.len() {
        if i >= k {
            if let Some(&count) = bst.get(&nums[i - k]) {
                if count == 1 {
                    bst.remove(&nums[i - k]);
                } else {
                    bst.insert(nums[i - k], count - 1);
                }
            }
        }
        *bst.entry(nums[i]).or_insert(0) += 1;
        if i >= k - 1 {
            if let Some((&max_value, _)) = bst.iter().rev().next() {
                result.push(max_value);
            }
        }
    }
    result
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut vec = Vec::new();
    let mut queue = Vec::new();
    let mut len = 0;
    for i in 0..nums.len() {
        while !queue.is_empty() && queue[0] + k <= i {
            queue.remove(0);
            len -= 1;
        }
        while !queue.is_empty() && nums[i] >= nums[queue[len - 1]] {
            queue.remove(len - 1);
            len -= 1;
        }
        queue.push(i);
        len += 1;
        if i >= k - 1 {
            vec.push(nums[queue[0]]);
        }
    }
    vec
}

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    //#[ignore]
    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    //#[ignore]
    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    //#[ignore]
    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}
