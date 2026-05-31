use std::io;

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack: Vec<char> = Vec::new();
        let mut res: Vec<String> = Vec::new();

        fn backtrack(
            open_count: i32,
            closed_count: i32,
            n: i32,
            res: &mut Vec<String>,
            stack: &mut Vec<char>,
        ) {
            if open_count == n && closed_count == n {
                res.push(stack.iter().collect());
                return;
            }

            if open_count < n {
                stack.push('(');
                backtrack(open_count + 1, closed_count, n, res, stack);
                stack.pop();
            }

            if closed_count < open_count {
                stack.push(')');
                backtrack(open_count, closed_count + 1, n, res, stack);
                stack.pop();
            }
        }
        backtrack(0, 0, n, &mut res, &mut stack);
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");
    let n = input.trim().parse().expect("Failed to pase");
    println!(
        "All the possible solution: {:?}",
        Solution::generate_parenthesis(n)
    );
}
