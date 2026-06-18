use std::io;

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut row = vec![1; n as usize];
        for _ in 1..m {
            let mut new_row = vec![1; n as usize];
            for j in (0..=(n - 2)).rev() {
                new_row[j as usize] = new_row[j as usize + 1] + row[j as usize];
            }
            row = new_row;
        }
        row[0]
    }
}

fn take_input() -> i32 {
    println!("Enter the Input");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let m = take_input();
    let n = take_input();
    // let m = 3;
    // let n = 2;
    println!("Total Possible paths are: {}", Solution::unique_paths(m, n));
}
