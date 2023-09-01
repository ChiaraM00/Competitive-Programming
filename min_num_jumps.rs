fn min_jumps(arr: &[i32], n: usize) -> i32 {
    if n==1 {
        return 0;
    }
    if arr[0]==0 {
        return -1;
    }
    let mut maxr = arr[0];
    let mut steps = arr[0];
    let mut jumps = 1;
    if arr[0]>=(n-1) as i32 {
        return 1;
    }
    for i in 1..n {
        maxr = std::cmp::max(maxr, arr[i] + i as i32);
        if maxr>=(n-1) as i32 {
            return jumps+1;
        }
        steps-=1;
        if steps==0 {
            if i as i32==maxr {
                return -1;
            }
            jumps+=1;
            steps=maxr-i as i32;
        }
    }
    -1
}
