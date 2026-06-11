struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut smallest = 1;
        for ele in nums {
            if ele > smallest {
                return smallest;
            }
            if ele == smallest {
                smallest += 1;
            }
        }
        smallest
    }
}

fn main() {
    let nums = vec![3, 4, -1, 1];
    print!("{}", Solution::first_missing_positive(nums));
}
