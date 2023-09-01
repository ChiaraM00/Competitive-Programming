impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut currSum = 0;
        let mut maxSum = i32::MIN;
        for i in (0..nums.len()) {
            currSum += nums[i];
            if currSum > maxSum {
                maxSum = currSum;
            }
            if currSum < 0 {
                currSum = 0;
            }
        }
    maxSum
    }
}