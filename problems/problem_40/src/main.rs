use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut res: HashSet<Vec<i32>> = HashSet::new();
        let mut cur_vec: Vec<i32> = Vec::new();
        fn backtrack(
            res: &mut HashSet<Vec<i32>>,
            start: usize,
            cur_vec: &mut Vec<i32>,
            cur_sum: i32,
            candidates: &Vec<i32>,
            target: i32,
            len: usize,
        ) {
            if cur_sum > target {
                return;
            }
            if cur_sum == target {
                res.insert(cur_vec.clone());
                return;
            }
            for i in start..len {
                if i > start && candidates[i] == candidates[i - 1] {
                    continue;
                }

                if candidates[i] > target {
                    break;
                }
                cur_vec.push(candidates[i]);

                backtrack(
                    res,
                    i + 1,
                    cur_vec,
                    cur_sum + candidates[i],
                    candidates,
                    target,
                    len,
                );
                cur_vec.pop();
            }
        }
        backtrack(
            &mut res,
            0,
            &mut cur_vec,
            0,
            &candidates,
            target,
            candidates.len(),
        );
        res.into_iter().collect()
    }
}

fn main() {
    let candidates = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    let target = 30;
    print!("{:?}", Solution::combination_sum2(candidates, target));
}
