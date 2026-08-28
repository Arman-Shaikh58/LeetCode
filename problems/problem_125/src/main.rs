use std::io;

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut string = String::new();
        for ch in s.chars() {
            if ch.is_alphanumeric() && ch != ' ' {
                string.push(ch.to_ascii_lowercase())
            }
        }
        let string = string.as_bytes();

        if string.len() == 0 {
            return true;
        }

        let mut left = 0;
        let mut right = string.len() - 1;

        while left < right {
            if string[left] != string[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

fn main() {
    println!("Enter a string");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", Solution::is_palindrome(input.trim().to_string()));
}
