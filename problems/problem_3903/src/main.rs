struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        for i in 0..n {
            let max = Solution::find_max(&nums[0..i + 1]);
            println!("{}", max);
            let min = Solution::find_min(&nums[i..n]);
            println!("{}", max);
            println!("{}", max - min);
            if (max - min) <= k {
                return i as i32;
            }
        }
        -1
    }
    pub fn find_max(nums: &[i32]) -> i32 {
        let mut max = i32::MIN;
        for ele in nums {
            if max < *ele {
                max = *ele;
            }
        }
        max
    }
    pub fn find_min(nums: &[i32]) -> i32 {
        let mut min = i32::MAX;
        for ele in nums {
            if min > *ele {
                min = *ele;
            }
        }
        min
    }
}

fn main() {
    let nums = vec![0, 0];
    let k = 0;
    println!("{}", Solution::first_stable_index(nums, k));
}
