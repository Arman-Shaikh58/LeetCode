struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len == 1 {
            if nums[0] == target {
                return 0 as i32;
            } else if nums[0] < target {
                return 1 as i32;
            } else {
                return 0;
            }
        }
        let mut left = 0;
        let mut right = nums.len();
        let mut mid = (left + right) / 2;

        while left != right {
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target {
                left = mid + 1;
                mid = (left + right) / 2;
            } else {
                right = mid;
                mid = (left + right) / 2;
            }
        }
        left as i32
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 4;
    println!("The index is: {}", Solution::search_insert(nums, target));
}
