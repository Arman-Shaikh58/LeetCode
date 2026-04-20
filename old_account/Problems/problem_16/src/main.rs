struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let len = nums.len();

        let mut closest_min = nums[0] + nums[1] + nums[2];
        for i in 0..len - 2 {
            let mut left = i + 1;
            let mut right = len - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if (sum-target).abs() < (closest_min-target).abs() { //here i misunderstood the question the question was actually to find element closest to target which mean distance = | start - end | (in the context of physic for formula) and i was finding it using minimun and maximun which  was never asked :( tricky questions good
                    
                    closest_min=sum;
                }
                if sum < target {
                    left+=1;
                }else {
                    right-=1;
                }
            }
        }
        closest_min
    }
}

fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    print!("Output: {}", Solution::three_sum_closest(nums, target));
}
