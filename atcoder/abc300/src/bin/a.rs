use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let _: i32 = iter.next().unwrap().parse().unwrap();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    let ans = a + b;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for (i, x) in v.iter().enumerate() {
        if *x == ans {
            println!("{}", i + 1);
            return;
        }
    }
}
