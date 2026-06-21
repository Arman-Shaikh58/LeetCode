use std::fmt::Write as _;
use std::io;
struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }

        let res = Self::count_and_say(n - 1);
        let mut rle = String::new();
        let mut run = (0, res.chars().next().unwrap());
        for ch in res.chars() {
            if ch == run.1 {
                run.0 += 1;
            } else {
                write!(rle, "{}{}", run.0, run.1).unwrap();
                run = (1, ch);
            }
        }
        write!(rle, "{}{}", run.0, run.1).unwrap();
        rle
    }
}
fn take_input() -> i32 {
    println!("Enter a Number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = take_input();
    println!("{}", Solution::count_and_say(n));
}
