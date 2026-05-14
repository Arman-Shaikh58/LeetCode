use std::io;

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut number: i32 = 0;
        let mut sign = 1;
        let mut started = false;

        for ch in s.chars() {
            if !started && ch == ' ' {
                continue;
            }

            if !started && (ch == '-' || ch == '+') {
                started = true;

                if ch == '-' {
                    sign = -1;
                }

                continue;
            }

            if ch.is_ascii_digit() {
                started = true;

                let digit = ch.to_digit(10).unwrap() as i32;
                if number > (i32::MAX - digit) / 10 {
                    return if sign == 1 { i32::MAX } else { i32::MIN };
                }

                number = number * 10 + digit;
            } else {
                break;
            }
        }

        number * sign
    }
}

fn main() {
    println!("Enter the string containing number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Take Input");
    let number = Solution::my_atoi(input.trim().to_string());
    println!("The number: {}", number);
}
