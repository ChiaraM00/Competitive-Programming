use std::io;

fn upper_bound(v: &Vec<i128>, p: i128) -> usize {
    let mut l = 0;
    let mut r = v.len();

    while l < r {
        let m =(l+r)/ 2;

        if v[m] <= p {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let N: Vec<i128> = input.split(' ').map(|x| x.trim().parse().unwrap()).collect();
    let n = N[0] as usize;
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let a: Vec<i128> = input2.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let mut sum: i128 = 0;
    for i in 0..n{
        sum+=a[i];
		
    }

    if (sum/3)*3!=sum { print!("{}",0); return ();}
    if sum==0 {
        let mut sum1: i128 = 0;
        let mut i = 0;
        let mut conto1: i128 =0;
        while i<n{
            if sum1==0{
            conto1+=1;
            }
            sum1+=a[i];
            i+=1;
        }
		let prod: i128 =(conto1-2)*(conto1-1)/2;
        print!("{}",prod);
        return ();
    
    
    }

    let mut sum1 = 0;
    let mut i = 0;
    let mut conto1 = Vec::new();
    while i<n{
        if sum1==sum/3{
            conto1.push((i-1) as i128); 
        }
        sum1+=a[i];
        i+=1;
    }

    let mut sum2 = 0;
    let mut j: i128 = (n as i128) -1;
    let mut conto2 = Vec::new();
    while j>=0{
        if sum2==sum/3{
            conto2.push(j);
        }
        sum2+=a[j as usize];
        j-=1;
    }
    conto2.reverse();
	let mut conto=0;
	for i in 0..conto1.len(){
	    conto+=conto2.len() - upper_bound(&conto2, conto1[i]);
	    
	}
	print!("{}", conto);
    
}
    
    