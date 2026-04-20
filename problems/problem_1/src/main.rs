// Problem 1: Two Sum

use std::{collections::HashMap, ops::RangeTo};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut list_map: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let tar: i32 = target - *num;

            if list_map.contains_key(&tar) {
                return vec![list_map[&tar], i as i32];
            }

            list_map.insert(*num, i as i32);
        }
        vec![]
    }
}
fn main() {}
