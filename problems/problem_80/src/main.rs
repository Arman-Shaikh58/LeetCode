struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;
        let mut dup: (i32, i32) = (nums[0], 1);
        let len = nums.len();
        let mut i = 0;
        while i < len {
            if nums[i] == dup.0 && dup.1 >= 0 {
                nums[k] = dup.0;
                k += 1;
                dup.1 -= 1;
            } else if nums[i] != dup.0 {
                dup = (nums[i], 0);
                nums[k] = dup.0;
                k += 1;
            }
            i += 1;
        }
        k as i32
    }
}

fn main() {
    let mut nums = vec![0, 0, 0, 1, 1, 1, 1, 2, 3, 3, 3, 4, 4];
    let k = Solution::remove_duplicates(&mut nums);
    println!("{:?}, {:?}", k, nums);
}
