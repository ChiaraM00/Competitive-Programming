fn merge(nums: &mut Vec<i32>, a: usize, b: usize, c: usize) -> usize{
    let mut sol = Vec::new();
    let mut i=a;
    let mut j=c+1;
    let mut nuovi=0;
    while i<=c && j <=b{
        if nums[i] <= nums[j]{
            sol.push(nums[i]);
            i+=1;
        } else{
            sol.push(nums[j]);
            nuovi+=c-i+1;
            j += 1;
        }
    }
    while i <= c{
        sol.push(nums[i]);
        i += 1;
    }
    while j <= b{
        sol.push(nums[j]);
        j += 1;
    }
    for i in a..b+1{
        nums[i]=sol[i-a];
    }
    nuovi   
}



fn mergesort(nums: &mut Vec<i32>, a: usize, b: usize) -> i32{
    if a==b{
        return 0;
    }
    let c=(a+b)/2;
    let v1=mergesort(nums,a,c);
    let v2=mergesort(nums,c+1,b);
    v1+v2+(merge(nums,a,b,c) as i32)
}

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let k = nums.len();
        let mut loc_inv=0;
        for i in 0..k-1{
            if nums[i+1]<nums[i]{
                loc_inv+=1;
            }
        }
        let glob_inv=mergesort(&mut nums.clone(),0,k-1);
        loc_inv==glob_inv
    }
}




