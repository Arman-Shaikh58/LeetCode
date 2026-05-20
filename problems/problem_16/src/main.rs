struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let len = nums.len();

        let mut closest_sum = nums[0] + nums[1] + nums[2];

        for i in 0..len - 2 {
            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }

                if sum == target {
                    return target;
                }

                if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        closest_sum
    }
}

fn main() {
    let nums = vec![1, 1, 1, 1];
    let target = 0;
    println!(
        "The Closest sum is: {}",
        Solution::three_sum_closest(nums, target)
    );
}
