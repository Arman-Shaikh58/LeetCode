use std::io;

struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let neg = (dividend < 0) ^ (divisor < 0);

        let mut dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();
        let mut quotient: i64 = 0;
        while dividend >= divisor {
            let mut temp = divisor;
            let mut multiple: i64 = 1;

            while dividend >= (temp << 1) {
                temp <<= 1;
                multiple <<= 1;
            }

            dividend -= temp;
            quotient += multiple;
        }

        if neg {
            -(quotient as i32)
        } else {
            quotient as i32
        }
    }
}

fn take_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    println!("Enter dividend");
    let dividend = take_input();
    println!("Enter divisor");
    let divisor = take_input();
    println!("The Quotent is: {}", Solution::divide(dividend, divisor));
}
