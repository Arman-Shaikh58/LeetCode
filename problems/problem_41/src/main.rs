struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        // first preprocess the array by modifying the element which is smaller than 1 and grater
        // than teh length of the array
        for i in 0..len {
            if nums[i] <= 0 || nums[i] >= len as i32 + 1 {
                nums[i] = len as i32 + 1;
            }
        }

        // use pigheon hole approch
        for i in 0..len {
            let num = nums[i].abs();
            if num == len as i32 + 1 {
                continue;
            }

            let seat = num - 1;
            if nums[seat as usize] > 0 {
                nums[seat as usize] = -nums[seat as usize];
            }
        }

        // find the first first_missing_positive
        for i in 0..len {
            if nums[i] > 0 {
                return i as i32 + 1;
            }
        }
        len as i32 + 1
    }
}

fn main() {
    let nums = vec![3, 4, -1, 1];
    print!("{}", Solution::first_missing_positive(nums));
}
