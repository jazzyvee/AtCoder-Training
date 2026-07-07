use std::{
    cmp::min,
    io::{self, Read},
};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let c: Vec<Vec<char>> = (0..h)
        .map(|_| iter.next().unwrap().chars().collect())
        .collect();
    let mut res: Vec<String> = Vec::new();
    let mut ans = vec![0; min(h, w)];
}

fn main_first() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let c: Vec<Vec<char>> = (0..h)
        .map(|_| iter.next().unwrap().chars().collect())
        .collect();
    let mut res: Vec<String> = Vec::new();
    let mut mark_as_used: Vec<Vec<bool>> = c.iter().map(|row| vec![false; row.len()]).collect();
    for n in (1..=min(h, w)).rev() {
        let mut count = 0;
        for i in n..(h - n) {
            for j in n..(w - n) {
                if c[i][j] == '#' && !mark_as_used[i][j] {
                    let mut is_ok = true;
                    for d in 1..=n {
                        let s: String = [
                            c[i - d][j - d],
                            c[i + d][j - d],
                            c[i + d][j + d],
                            c[i - d][j + d],
                        ]
                        .iter()
                        .collect();
                        if s != "####" {
                            is_ok = false;
                            break;
                        }
                    }
                    if is_ok {
                        count += 1;
                        mark_as_used[i][j] = true;
                    }
                }
            }
        }
        res.push(count.to_string());
    }
    // let res = res.re;
    res.reverse();
    println!("{}", res.join(" "));
}
