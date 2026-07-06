struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = Vec::new();

        fn backtrack(res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, current: &mut Vec<i32>, i: i32) {
            if i >= nums.len() as i32 {
                res.push(current.clone());
                return;
            }

            //left decision
            current.push(nums[i as usize]);
            backtrack(res, nums, current, i + 1);

            current.pop();
            backtrack(res, nums, current, i + 1);
        }
        backtrack(&mut res, &nums, &mut current, 0);
        res
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", Solution::subsets(nums));
}
