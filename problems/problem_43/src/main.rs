use std::{io, mem::swap};

struct Solution;

impl Solution {
    pub fn multiply(mut num1: String, mut num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        if num1.len() < num2.len() {
            swap(&mut num1, &mut num2);
        }
        let mut res = String::new();

        for (i, ch) in num2.chars().rev().enumerate() {
            let mut cur = String::new();
            let mut carray: u8 = 0;
            for n1 in num1.chars().rev() {
                let multi = Solution::stoi(ch) * Solution::stoi(n1) + carray;
                carray = (multi) / 10;
                cur.push(((multi % 10) + 48) as char);
            }
            if carray > 0 {
                cur.push((carray + 48) as char);
            }
            cur = cur.chars().rev().collect();
            cur.push_str(&"0".repeat(i));

            if res.len() == 0 {
                res = cur;
            } else {
                let mut carry = 0;
                let mut sum_res = String::new();

                let res_chars: Vec<char> = res.chars().collect();
                let cur_chars: Vec<char> = cur.chars().collect();

                let mut i = res_chars.len() as i32 - 1;
                let mut j = cur_chars.len() as i32 - 1;

                while i >= 0 || j >= 0 || carry > 0 {
                    let a = if i >= 0 {
                        res_chars[i as usize].to_digit(10).unwrap() as i32
                    } else {
                        0
                    };

                    let b = if j >= 0 {
                        cur_chars[j as usize].to_digit(10).unwrap() as i32
                    } else {
                        0
                    };

                    let sum = a + b + carry;
                    carry = sum / 10;

                    sum_res.push(char::from_digit((sum % 10) as u32, 10).unwrap());

                    i -= 1;
                    j -= 1;
                }

                res = sum_res.chars().rev().collect();
            }
        }
        res
    }
    pub fn stoi(ch: char) -> u8 {
        ch as u8 - 48
    }
}

fn take_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Faield to tajke inhout");
    input.trim().to_string()
}
fn main() {
    println!("Enter number 1");
    let num1 = take_input();
    println!("Enter number 2");
    let num2 = take_input();
    println!("{}", Solution::multiply(num1, num2));
}
