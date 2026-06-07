// kadane's Algorithm this algo is used to find the max sum in a subarray

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut curr = 0;
        for element in nums {
            curr += element;
            max = max.max(curr);
            if curr < 0 {
                curr = 0;
            }
        }
        max
    }
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximhum is: {}", Solution::max_sub_array(nums));
}
