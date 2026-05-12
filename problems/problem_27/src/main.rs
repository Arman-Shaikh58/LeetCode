struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}
fn main() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    println!(
        "Size of the Array: {}",
        Solution::remove_element(&mut nums, val)
    );
    for element in &nums {
        println!("{}", element);
    }
}
