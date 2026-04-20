struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // Edge case: overflow
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        // Convert to i64 for the edge case bruh
        let mut a = dividend as i64;
        let mut b = divisor as i64;

        let negative = (a < 0) ^ (b < 0); //XOR logic 

        a = a.abs();
        b = b.abs();

        let mut result: i64 = 0;

        while a >= b {
            let mut temp = b;
            let mut multiple = 1;

            while a >= (temp << 1) {
                temp <<= 1;
                multiple <<= 1;
            }

            a -= temp;
            result += multiple;
        }

        if negative {
            -result as i32
        } else {
            result as i32
        }
    }
}

fn main(){
    let dividend = 10;
    let divisor = -3;
    print!("The quotent is {}",Solution::divide(dividend, divisor))
}