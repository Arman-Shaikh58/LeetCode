struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; triangle.len() + 1];

        for i in (0..triangle.len()).rev() {
            for (j, ele) in triangle[i].iter().enumerate() {
                dp[j] = *ele + dp[j].min(dp[j + 1]);
            }
        }
        dp[0]
    }
}

fn main() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    let triangle = vec![vec![2], vec![3, 4]];
    let triangle = vec![vec![2]];
    let triangle = vec![vec![2], vec![-3, 4], vec![100, 99, 7], vec![4, 1, -8, 3]];
    println!("{}", Solution::minimum_total(triangle));
}
