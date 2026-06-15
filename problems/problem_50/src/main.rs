struct Solution;

impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut n = n as i64;

        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        let mut res = 1.0;

        while n > 0 {
            if n % 2 == 1 {
                res *= x;
            }

            x *= x;
            n /= 2;
        }

        res
    }
}

fn main() {
    println!("{}", Solution::my_pow(2.1, 3));
    println!("{}", Solution::my_pow(0.0, 1));
    println!("{}", Solution::my_pow(2.0, -2));
}
