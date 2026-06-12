// this solution from the neetcode was awesome i never thought obout that

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut goal = len - 1;
        for i in (0..len).rev() {
            if i + nums[i] as usize >= goal {
                goal = i;
            }
        }
        goal == 0
    }
}

fn main() {
    let nums = vec![3, 2, 1, 0, 4];
    println!("{}", Solution::can_jump(nums));
}
