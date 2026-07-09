use nalgebra::min;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Chars
    }
    let t = s;
    let x_idx_vec = x_vec(&t);
    let mut max_count = 0;
    for i in 0..(x_idx_vec.len() * m - k) {
        let mut k_remained = k;
        let first_skip = i;
        let x_skip_count = 0;
        let mut count = 0;
        while k_remained > 0 {
            let mut t_clone = t.clone();
            for j in 0..min(x_idx_vec.len(), k_remained) {
                t_clone[x_idx_vec[j]] = 'o';
                k_remained -= 1;
            }
            count += o_count(&t_clone);
        }
        if max_count < count {
            max_count = count;
        }
    }
    for i in 0..(x_idx_vec.len() - k) {
        let mut t_clone = t.clone();
        for j in i..(i + k) {
            t_clone[x_idx_vec[j]] = 'o';
        }
        let count = o_count(&t_clone);
        if max_count < count {
            max_count = count;
        }
    }
    println!("{}", max_count);
}

fn sub_main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Chars
    }
    let t = s.repeat(m);
    let x_idx_vec = x_vec(&t);
    let mut max_count = 0;
    for i in 0..(x_idx_vec.len() - k) {
        let mut t_clone = t.clone();
        for j in i..(i + k) {
            t_clone[x_idx_vec[j]] = 'o';
        }
        let count = o_count(&t_clone);
        if max_count < count {
            max_count = count;
        }
    }
    println!("{}", max_count);
}
// xのindexリスト
fn x_vec(t: &[char]) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    for (i, &v) in t.iter().enumerate() {
        if v == 'x' {
            res.push(i);
        }
    }
    res
}

fn o_count(t: &Vec<char>) -> usize {
    let mut count = 0;
    let mut max_count = 0;
    for &i in t {
        if i == 'o' {
            count += 1;
        } else {
            if max_count < count {
                max_count = count;
            }
            count = 0;
        }
    }
    max_count
}
