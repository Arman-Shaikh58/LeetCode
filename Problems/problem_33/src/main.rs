struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];

            if mid_val == target {
                return mid;
            }

            // Left half is sorted
            if nums[left as usize] <= mid_val {
                if nums[left as usize] <= target && target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            // Right half is sorted
            else {
                if mid_val < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        -1
    }
}

fn main(){
    let nums = vec![4,5,6,7,0,1,2];
    let target = 0;
    println!("Output: {}",Solution::search(nums, target))
}