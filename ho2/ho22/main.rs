use std::io;

fn main() {
 
    let mut my_string = String::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let my_string: Vec<usize> = my_string.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let n = my_string[0];
	let m = my_string[1];
	let k = my_string[2];
 
    let mut my_string = String::new();
	let mut nums: Vec<i64> = Vec::new();
	io::stdin().read_line(&mut my_string).unwrap();
    let nums: Vec<i64> = my_string.trim().split(' ').map(|x| x.parse().unwrap()).collect();
	
	
	let mut op: Vec<(usize,usize,i64)> = Vec::new();
	for _ in 0..m {
        let mut _st = String::new();
        io::stdin().read_line(&mut _st).unwrap();
        let _st: Vec<usize> = _st.trim().split(' ').map(|x| x.parse().unwrap()).collect();
       op.push((_st[0], _st[1], _st[2] as i64));
    }
	
	
	let mut queries: Vec<(usize,usize)> = Vec::new();
	for _ in 0..k {
        let mut _st = String::new();
        io::stdin().read_line(&mut _st).unwrap();
        let _st: Vec<usize> = _st.trim().split(' ').map(|x| x.parse().unwrap()).collect();
       queries.push((_st[0], _st[1]));
    }

    let mut queries_start = vec![0; m];
    let mut queries_end = vec![0; m];

    for i in 0..k {
        queries_start[queries[i].0 - 1] += 1;
        queries_end[queries[i].1 - 1] += 1;
    }

    let mut op_count = vec![0; m];
    op_count[0] = queries_start[0];
    let mut count = queries_start[0];

    for j in 1..m {
        count -= queries_end[j-1];
        count += queries_start[j];
        op_count[j] = count;
    }

    for j0 in 0..m {
        op[j0].2 *= op_count[j0];
    }
   
    let mut op_start = vec![0; n];
    let mut op_end = vec![0; n];
    for i in 0..m {
        op_start[op[i].0 - 1] += op[i].2;
        op_end[op[i].1 - 1] += op[i].2;
    }
    let mut val_to_add = vec![0; n];
    val_to_add[0] = op_start[0];
    let mut count = op_start[0];

    for j in 1..n {
        count -= op_end[j-1];
        count += op_start[j];
        val_to_add[j] = count;
    }
    
    for i0 in 0..n {
        print!("{} ", val_to_add[i0] + nums[i0]);
    }
}