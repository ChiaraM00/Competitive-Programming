fn lcs(x: usize, y: usize, s1: &str, s2: &str) -> usize {
    let mut tab = vec![vec![0; y + 1]; x + 1];
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..y+1 {
        for j in 1..x+1 {
            if s1_chars[j-1] == s2_chars[i-1] {tab[j][i] = tab[j-1][i-1]+1;}
            else {tab[j][i]=std::cmp::max(tab[j-1][i],tab[j][i-1]);}
        }
    }
    tab[x][y]
}

