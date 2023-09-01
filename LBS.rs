use std::cmp::Ordering;

fn longest_bitonic_sequence(a: Vec<i32>) -> i32 {
    let mut pila = Vec::new(); 
    let mut lis = Vec::new();

    for &num in &a {
        match pila.binary_search(&num) {
            Ok(index) => pila[index] = num,
            Err(index) => pila.insert(index, num),
        }
        lis.push(pila.len() as i32);
    }

    let a_reverse = a.iter().rev().cloned().collect::<Vec<_>>();

    pila = Vec::new(); 
    let mut lds = Vec::new();

    for &num in &a_reverse {
        match pila.binary_search(&num) {
            Ok(index) => pila[index] = num,
            Err(index) => pila.insert(index, num),
        }
        lds.push(pila.len() as i32);
    }

    let lbs = lis.iter().zip(lds.iter().rev()).map(|(&i, &j)| i + j - 1).collect::<Vec<_>>();
    *lbs.iter().max().unwrap()
}




fn longest_bitonic_sequence(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut lis = vec![1; n];
    
    for i in 1..n {
        for j in 0..i {
            if a[j] < a[i] {
                lis[i] = std::cmp::max(lis[i], lis[j] + 1);
            }
        }
    }
    
    let mut a_rev = a.clone();
    a_rev.reverse();
    
    let mut lds = vec![1; n];
    for i in 1..n {
        for j in 0..i {
            if a_rev[j] < a_rev[i] {
                lds[i] = std::cmp::max(lds[i], lds[j] + 1);
            }
        }
    }
    
    let lbs: Vec<i32> = lis.iter().zip(lds.iter().rev()).map(|(i, j)| i + j - 1).collect();
    *lbs.iter().max().unwrap
}