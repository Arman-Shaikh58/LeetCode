struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut cur: Vec<i32> = Vec::new();
        let len = nums.len();

        fn backtrack(res: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, len: usize, nums: &Vec<i32>) {
            if cur.len() == len {
                res.push(cur.clone());
                return;
            }
            for ele in nums {
                if cur.contains(ele) {
                    continue;
                }
                cur.push(*ele);
                backtrack(res, cur, len, nums);
                cur.pop();
            }
        }
        backtrack(&mut res, &mut cur, len, &nums);

        res
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    print!("{:?}", Solution::permute(nums));
}
