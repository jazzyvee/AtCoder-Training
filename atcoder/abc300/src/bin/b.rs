use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    // let mut grid_a: Vec<Vec<i32>> = Vec::new();
    // let mut grid_b: Vec<Vec<i32>> = Vec::new();
    let grid_a: Vec<Vec<char>> = (0..h)
        .map(|_| iter.next().unwrap().chars().collect())
        .collect();
    let grid_b: Vec<Vec<char>> = (0..h)
        .map(|_| iter.next().unwrap().chars().collect())
        .collect();
    // for _ in 0..h {
    //     io::stdin().read_line(&mut buf).unwrap();
    //     let row: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    //     grid_a.push(row);
    // }
    // for _ in 0..h {
    //     io::stdin().read_line(&mut buf).unwrap();
    //     let row: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    //     grid_b.push(row);
    // }

    // if (grid_a == grid_b) {
    //     println!("Yes")
    // }
    for s in 0..h {
        let mut tmp_a = grid_a.clone();
        // let mut tmp_b = grid_b.clone();
        tmp_a.rotate_right(s);
        for t in 0..w {
            let mut candidate = tmp_a.clone();
            for row in candidate.iter_mut() {
                row.rotate_right(t);
            }
            if grid_b == candidate {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

// fn same_grid(g1: Vec<Vec<i32>>, g2: Vec<Vec<i32>>) -> bool {
//     g1 == g2
// }
