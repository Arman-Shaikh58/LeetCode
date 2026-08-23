struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit: i32 = 0;
        let mut holding_stock_price = i32::MAX;

        for i in 0..prices.len() {
            // println!("{profit}");
            if i == prices.len() - 1 {
                if holding_stock_price == i32::MAX {
                    break;
                }
                profit += prices[i] - holding_stock_price;
                break;
            }
            if prices[i] < prices[i + 1] {
                holding_stock_price = holding_stock_price.min(prices[i]);
            } else if prices[i] > prices[i + 1] && holding_stock_price != i32::MAX {
                profit += prices[i] - holding_stock_price;
                holding_stock_price = i32::MAX;
            }
        }
        profit
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let prices = vec![1, 2, 3, 4, 5];
    let prices = vec![7, 6, 4, 3, 1];
    let prices = vec![1, 4, 7, 8, 6, 4];
    println!("{}", Solution::max_profit(prices));
}
