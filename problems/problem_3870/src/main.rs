use std::io;

struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        if n < 1000 {
            return 0;
        }
        (n - 1000) + 1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", Solution::count_commas(input.trim().parse().unwrap()));
}
