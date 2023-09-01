use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let n: usize = input.trim().parse().expect("");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let char_vector: Vec<char> = input.trim().chars().collect();

    let mut total = vec![(0,0,0);n+1];
    let mut count = 1;
    for i in 1..n+1 {
        if char_vector[i-1] == 'R' {
            total[i].0 = total[i-1].0 + count;
            total[i].1 = total[i-1].1;
            total[i].2 = total[i-1].2;
        } else if char_vector[i-1] == 'W' {
            total[i].1 = total[i-1].1 + total[i-1].0;
            total[i].0 = total[i-1].0;
            total[i].2 = total[i-1].2;
        } else if char_vector[i-1] == 'G' {
            total[i].2 = total[i-1].2 + total[i-1].1;
            total[i].1 = total[i-1].1;
            total[i].0 = total[i-1].0;
        } else {
            total[i].0 = 3*total[i-1].0 + count;
            total[i].1 = 3*total[i-1].1 + total[i-1].0;
            total[i].2 = 3*total[i-1].2 + total[i-1].1;
            count *= 3;
        }
    }    
    println!("{}", total[n].2);
}
