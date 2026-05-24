struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[k - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}

fn main() {
    let mut nums = vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!(
        "Vector with Unique elements: {}",
        Solution::remove_duplicates(&mut nums)
    );
    for element in nums {
        print!("{element} ");
    }
}
