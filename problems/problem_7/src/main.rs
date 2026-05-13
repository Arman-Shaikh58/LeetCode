use std::io;

struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x_cpy = x;
        let is_positive: bool;

        if x < 0 {
            is_positive = false;

            // handle i32::MIN
            if x == i32::MIN {
                return 0;
            }

            x_cpy *= -1;
        } else {
            is_positive = true;
        }

        let mut reversed: i32 = 0;

        while x_cpy != 0 {
            let digit = x_cpy % 10;

            // safe multiplication
            match reversed.checked_mul(10) {
                Some(val) => reversed = val,
                None => return 0,
            }

            // safe addition
            match reversed.checked_add(digit) {
                Some(val) => reversed = val,
                None => return 0,
            }

            x_cpy /= 10;
        }

        if !is_positive {
            reversed *= -1;
        }

        reversed
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");
    let x = input.trim().parse().expect("Failed to take input");
    print!("{}", Solution::reverse(x));
}
