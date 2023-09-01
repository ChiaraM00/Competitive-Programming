fn equal_partition(arr: Vec<i32>) -> bool {
    let sum: i32 = arr.iter().sum();
    if sum % 2 == 1 {
        return false;
    }
    let ref_sum = sum / 2;
    let mut dp = vec![vec![false; (ref_sum + 1) as usize]; (arr.len() + 1) as usize];

    for i in 0..=arr.len() {
        dp[i][0] = true;
    }

    for i in 1..=arr.len() {
        for j in 1..=ref_sum {
            dp[i][j as usize] = dp[i - 1][j as usize] || (j >= arr[i - 1] && dp[i - 1][(j - arr[i - 1]) as usize]);
        }
    }

    dp[arr.len()][ref_sum as usize]
}
