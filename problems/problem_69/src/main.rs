use std::io;

struct Solution;

impl Solution {
    pub fn my_sqrt(n: i32) -> i32 {
        //this is newtons formula for approximation of the square root
        if n == 0 {
            return 0;
        }
        if n == 1 || n == 2 || n == 3 {
            return 1;
        }

        let mut x = n / 3;
        let mut new_x;
        for _ in 0..20 {
            new_x = (x + (n / x)) / 2;
            x = new_x
        }
        x
    }
}

fn take_input() -> i32 {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let x = take_input();
    println!("The square of {} is: {}", x, Solution::my_sqrt(x));
}
