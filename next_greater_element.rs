impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let k = nums.len();
        let mut M = *nums.iter().max().unwrap();
        let mut numeri = [&nums[..], &nums[..]].concat();
        let mut vec = Vec::new();
        let mut queue = Vec::new();
        let mut len = 0;
        for i in (0..k+k){
            vec.push(0);
            while !queue.is_empty() && numeri[i] > numeri[queue[len-1]] {
                vec[queue[len-1]]=numeri[i];
                queue.remove(len-1);
                len -= 1;
            }
            queue.push(i);
            len +=1;
            if numeri[i] == M {
                vec[queue[len-1]]=-1;
                queue.remove(len-1);
                len -=1;
            }
        }    
        vec[..k].to_vec()
    }
}


