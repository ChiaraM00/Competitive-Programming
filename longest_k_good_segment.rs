use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let dim: Vec<usize> = input.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let a: Vec<i32> = input2.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let n = dim[0];
    let m = dim[1];

    let mut freq = vec![0; 1000000 + 5];

    let mut unici = 0;
    let mut max = 0;
    let mut inizio = 0;
    let mut fine = 0;
    let mut q=0;

    for i in 0..n {
        if freq[a[i] as usize] == 0 {
            unici += 1;
        }
        freq[a[i] as usize] += 1;
        while q<i && unici > m {
            if freq[a[q] as usize] == 1 {
                unici -= 1;
            }
            freq[a[q] as usize] -= 1;
            q+=1;
        }

        if i-q+1 > max {
            max = i-q+1;
            inizio = i - max + 1;
            fine = i;
        }
    }
    println!("{} {}", inizio + 1, fine + 1);
}
