use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Chars
    }
    println!("{}", x_count(&s));
}

fn x_count(t: &Vec<char>) -> usize {
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
