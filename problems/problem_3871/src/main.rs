use std::io;

struct Solution;

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut res = 0;
        let mut start = 1000;
        let mut commas = 1;

        while start <= n {
            let end = (start * 1000 - 1).min(n);

            res += (end - start + 1) * commas;

            start *= 1000;
            commas += 1;
        }

        res
    }
}

fn main() {
    // println!("{}", i64::MAX);
    // println!("{}", (10 as i64).pow(15));
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!(
        "Commas {}",
        Solution::count_commas(input.trim().parse().unwrap())
    );
}
