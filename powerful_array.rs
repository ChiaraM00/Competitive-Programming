use std::collections::HashMap;
use std::io;

fn calculate_power(arr: &Vec<i64>, queries: &Vec<(usize, usize)>) {
    let n = arr.len();
    let mut blocks: Vec<Vec<(usize, usize)>> = vec![vec![]; (n as f64).sqrt() as usize + 1];

    for &(l, r) in queries {
        blocks[l / ((n as f64).sqrt() as usize + 1)].push((l, r));
    }

    for b in &mut blocks {
        b.sort_by_key(|&(_, r)| r);
    }

    let mut ris: HashMap<(usize, usize), i64> = HashMap::new();
    let mut power = arr[0];

    let max_value = *arr.iter().max().unwrap();
    let mut freq: Vec<i64> = vec![0; (max_value + 1) as usize];
    freq[arr[0] as usize] += 1;

   

    let mut left = 0;
    let mut right = 0;
    for t in 0..blocks.len() {
        for &(l, r) in &blocks[t] {
            let mut l = l - 1;
            let mut r = r - 1;

            while right < r {
                right += 1;
                if left <= right {
                  let num = arr[right];
                  power += (2 * freq[num as usize] + 1) * num;
                        freq[num as usize] += 1;
                    
                }
            }

            while left < l {
                left += 1;
                let num = arr[left - 1];
                  power -= (2 * freq[num as usize] - 1) * num;
                  freq[num as usize] -= 1;
            }

            while right > r {
                right -= 1;
                let num = arr[right + 1];
                  power -= (2 * freq[num as usize] - 1) * num;
                  freq[num as usize] -= 1;
            }

            while left > l {
                left -= 1;
                if left <= right {
                    let num = arr[left];
                    power += (2 * freq[num as usize] + 1) * num;
                    freq[num as usize] += 1;
                }
            }

            ris.insert((l + 1, r + 1), power);
        }
    }

    for &(a, b) in queries {
        println!("{}", ris[&(a, b)]);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let _n: i64 = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut queries: Vec<(usize, usize)> = Vec::new();
    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        queries.push((a, b));
    }

    calculate_power(&arr, &queries);
}
