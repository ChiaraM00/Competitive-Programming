use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.parse().unwrap();
    let mut ris = Vec::new();
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let m: i32 = input.parse().unwrap();
        let mut a_input = String::new();
        io::stdin().read_line(&mut a_input).unwrap();
        let a: Vec<i32> = a_input.split(' ').map(|x| x.parse().unwrap()).collect();
        let mut diff = Vec::new();
        diff.push(a[0]);
        for i in 0..(m - 1) as usize {
            diff.push(a[i + 1] - a[i]);
        }
        let mut k = *diff.iter().max().unwrap();
        ris.push(k);
        for j in 0..m as usize {
            if diff[j] == k {
                k -= 1;
            } else if diff[j] > k {
                ris[i as usize] += 1;
                break;
            }
        }
    }
    for i in 0..n as usize {
        println!("Case {}: {}", i + 1, ris[i]);
    }
}
