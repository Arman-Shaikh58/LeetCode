struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        Self::k_sum(&nums, 0, 4, target as i64)
    }

    fn k_sum(nums: &[i32], start: usize, k: usize, target: i64) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let n = nums.len();

        if start >= n || n - start < k {
            return res;
        }

        if k == 2 {
            let mut left = start;
            let mut right = n - 1;

            while left < right {
                let sum = nums[left] as i64 + nums[right] as i64;

                if sum == target {
                    res.push(vec![nums[left], nums[right]]);

                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }

            return res;
        }

        for i in start..=(n - k) {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }

            let subsets = Self::k_sum(nums, i + 1, k - 1, target - nums[i] as i64);

            for mut subset in subsets {
                let mut current = vec![nums[i]];
                current.append(&mut subset);
                res.push(current);
            }
        }

        res
    }
}

fn main() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    print!("{:?}", Solution::four_sum(nums, target));
}
