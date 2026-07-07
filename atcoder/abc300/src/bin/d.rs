use num::pow;
use num_integer::Roots;
use proconio::input;

fn prime_list(n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let mut v: Vec<usize> = vec![0; n];
    for i in 2..n {
        if v[i] == 1 {
            continue;
        }
        res.push(i);
        for j in (i..n).step_by(i) {
            v[j] = 1;
        }
    }
    res
}

fn main() {
    // first_try();
    input! {
        n:usize
    }
    let primes: Vec<usize> = prime_list(3 * 10usize.pow(5));
    let mut ans = 0;
    // println!("{}", n.sqrt().sqrt());
    for (i, &a) in primes.iter().enumerate() {
        let val1 = a * a;
        for &b in &primes[i + 1..] {
            let val2 = val1 * b;
            if val2 > n / (b + 1) {
                break;
            }
            let c_max = n / val2;
            let c_max = c_max.sqrt();
            if c_max <= b {
                break;
            }
            let lo = primes.partition_point(|&x| x <= b);
            let hi = primes.partition_point(|&x| x <= c_max);
            ans += hi - lo;
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

fn first_try() {
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
