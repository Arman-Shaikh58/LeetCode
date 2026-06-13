use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for ele in &nums {
            *hashmap.entry(*ele).or_insert(0) += 1;
        }
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut cur: Vec<i32> = Vec::new();
        let len = nums.len();

        fn backtrack(
            res: &mut Vec<Vec<i32>>,
            cur: &mut Vec<i32>,
            len: usize,
            nums: &Vec<i32>,
            hashmap: &mut HashMap<i32, i32>,
        ) {
            if cur.len() == len {
                res.push(cur.clone());
                return;
            }
            for (&num, &count) in hashmap.clone().iter() {
                if count <= 0 {
                    continue;
                }

                *hashmap.get_mut(&num).unwrap() -= 1;

                cur.push(num);
                backtrack(res, cur, len, nums, hashmap);
                cur.pop();

                *hashmap.get_mut(&num).unwrap() += 1;
            }
        }
        backtrack(&mut res, &mut cur, len, &nums, &mut hashmap);

        res
    }
}

fn main() {
    let nums = vec![1, 1, 2];
    println!("{:?}", Solution::permute(nums));
}
