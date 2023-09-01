use std::io;

fn main() {
 
    let mut my_string = String::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let my_string: Vec<usize> = my_string.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let n = my_string[0];
	let D = my_string[1];
 
	let mut cities: Vec<Vec<i64>> = Vec::new();
	for _ in 0..n {
        let mut _st = String::new();
        io::stdin().read_line(&mut _st).unwrap();
        let _st: Vec<i64> = _st.trim().split(' ').map(|x| x.parse().unwrap()).collect();
       cities.push(_st);
    }

    let mut plan = vec![vec![0;n+1];D+1];

    for j in 1..D+1 {
        for i in 1..n+1{
            let mut my_max = (0,0);
                for removed_days in 0..j+1{
                    
                    if plan[j-removed_days][i-1] + cities[i-1][0..removed_days].iter().sum::<i64>() > my_max.1 {
                        my_max = (removed_days,plan[j-removed_days][i-1] + cities[i-1][0..removed_days].iter().sum::<i64>());
                    }
                }
                plan[j][i] = my_max.1;
        }
    }
    println!("{}",plan[D][n]);

}