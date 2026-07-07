use num::pow;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    // println!("{}", n.sqrt().sqrt());
    for a in 2..=n.sqrt().sqrt() {
        let val1 = a.pow(2);
        if !is_prime(a) {
            continue;
        }
        for b in (a + 1)..=(n / val1).cbrt() {
            let val2 = val1 * b;
            if !is_prime(b) {
                continue;
            }
            for c in (b + 1)..=(n / val2).sqrt() {
                let val3 = val2 * pow(c, 2);
                if val3 > n {
                    break;
                }
                if is_prime(c) {
                    // println!("{} {} {}", a, b, c);
                    ans += 1;
                }
            }
        }
    }
    print!("{}", ans);
}

fn is_prime(n: usize) -> bool {
    for i in 2..=n.sqrt() {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}
