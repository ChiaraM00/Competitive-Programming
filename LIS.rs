use std::cmp::Ordering;

fn longest_subsequence(arr: Vec<i32>) -> i32 {
    let mut pila: Vec<i32> = Vec::new();

    for &num in &arr {
        match pila.binary_search(&num) {
            Ok(index) => pila[index] = num,
            Err(index) => {
                if index == pila.len() {
                    pila.push(num);
                } else {
                    pila[index] = num;
                }
            }
        }
    }

    pila.len() as i32
}


fn longest_subsequence(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut lis = vec![1; n];
    for i in 1..n {
        for j in 0..i {
            if a[j] < a[i] {
                lis[i] = std::cmp::max(lis[i], lis[j] + 1);
            }
        }
    }
    *lis.iter().max().unwrap()
}
