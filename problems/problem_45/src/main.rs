struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut jump = 0;
        let mut l = 0;
        let mut r = 0;

        while r < len - 1 {
            let mut farthest = nums[r];
            for i in l..r + 1 {
                farthest = farthest.max(i as i32 + nums[i]);
            }
            l = r + 1;
            r = farthest as usize;
            jump += 1;
        }
        jump
    }
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    println!("{}", Solution::jump(nums));
}
