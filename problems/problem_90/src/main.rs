struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = Vec::new();

        nums.sort_unstable();
        fn backtrack(
            nums: &Vec<i32>,
            current: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
            start: usize,
        ) {
            res.push(current.clone());
            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }

                current.push(nums[i]);
                backtrack(nums, current, res, i + 1);
                current.pop();
            }
        }
        backtrack(&nums, &mut current, &mut res, 0);
        res
    }
}

fn main() {
    let nums = vec![1, 2, 2];
    println!("{:?}", Solution::subsets_with_dup(nums));
}
