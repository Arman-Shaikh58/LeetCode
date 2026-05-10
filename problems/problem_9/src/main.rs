use std::io;

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut x_cpy = x;
        let mut palidrome: i32 = 0;

        while x_cpy != 0 {
            let last_digit = x_cpy % 10;
            palidrome = palidrome * 10 + last_digit;
            x_cpy /= 10;
        }

        if palidrome != x { false } else { true }
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter  a  number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");
    let x = input.trim().parse().expect("Failed To parse the number");
    if Solution::is_palindrome(x) {
        println!("{x} is an palidrome");
    } else {
        println!("{x} is not an palidrome");
    }
}
