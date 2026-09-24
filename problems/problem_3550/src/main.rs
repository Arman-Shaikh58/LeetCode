struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        let cal_sum = |mut n: i32| {
            let mut sum = 0;
            while n != 0 {
                sum += n % 10;
                n /= 10;
            }
            sum
        };

        for i in 0..nums.len() {
            if i as i32 == cal_sum(nums[i] as i32) {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("{}", Solution::smallest_index(nums));
}
