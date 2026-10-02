use std::io::{self, Cursor};

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        fn backtrack(res: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>, j: i32, n: i32, k: i32) {
            if curr.len() == k as usize {
                res.push(curr.clone());
                return;
            }

            for num in j..=n {
                curr.push(num);
                backtrack(res, curr, num + 1, n, k);
                curr.pop();
            }
        }

        let mut curr: Vec<i32> = Vec::new();
        backtrack(&mut res, &mut curr, 1, n, k);

        res
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the Value of n");
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    println!("Enter the value of k");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let k = input.trim().parse().unwrap();
    println!("Solution: {:?}", Solution::combine(n, k));
}
