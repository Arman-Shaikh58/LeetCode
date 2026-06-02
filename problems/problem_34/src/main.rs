struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        if len == 0 {
            return vec![-1, -1];
        }
        if len == 1 && nums[0] != target {
            return vec![-1, -1];
        }
        if len == 1 && nums[0] == target {
            return vec![0, 0];
        }

        let mut left: usize = 0;
        let mut right = len - 1;
        let mut first: i32;
        let mut last: i32;
        let mut mid = left + (right - left) / 2;

        while left <= right {
            if nums[mid] == target {
                first = mid as i32;
                last = mid as i32;
                while first > 0 && nums[first as usize] == nums[(first - 1) as usize] {
                    first -= 1;
                }
                while last < len as i32 - 1 && nums[last as usize] == nums[(last + 1) as usize] {
                    last += 1;
                }
                return vec![first, last];
            }

            if nums[mid] < target {
                left = mid + 1;
                mid = left + (right - left) / 2;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
                mid = left + (right - left) / 2;
            }
        }
        vec![-1, -1]
    }
}

fn main() {
    let nums = vec![2, 2];
    let target = 2;
    println!(
        "THe last &  first is: {:?}",
        Solution::search_range(nums, target)
    );
}
