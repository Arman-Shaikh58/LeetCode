struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut min = i32::MAX;
        for price in prices {
            min = min.min(price);
            res = res.max(price - min);
        }
        res
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", Solution::max_profit(prices));
}
