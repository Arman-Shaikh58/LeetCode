struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut cur_vec: Vec<i32> = Vec::new();
        fn backtrack(
            res: &mut Vec<Vec<i32>>,
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
                res.push(cur_vec.clone());
                return;
            }
            for i in start..len {
                cur_vec.push(candidates[i]);

                backtrack(
                    res,
                    i,
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
        res
    }
}

fn main() {
    let candidates = vec![2, 3, 5];
    let target = 8;
    println!(
        "Answer: {:?}",
        Solution::combination_sum(candidates, target)
    );
}
