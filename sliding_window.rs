impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut vec = Vec::new();
        let mut queue = Vec::new();
        let mut len = 0;
        for i in (0..nums.len()){
            while !queue.is_empty() && queue[0] + k <= i {
                queue.remove(0);
                len -=1;
            }
            while !queue.is_empty() && nums[i] >=nums[queue[len-1]] {
                queue.remove(len-1);
                len -=1;
            }
            queue.push(i);
            len +=1;
            if (i >= k - 1) {
                vec.push(nums[queue[0]]);
            }
        }
        vec
    }
}

