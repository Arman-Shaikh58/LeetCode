struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut best_len = 0;
        let mut stack: Vec<i32> = vec![-1];

        for (i, ch) in s.chars().enumerate() {
            if ch == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    let len = i as i32 - *stack.last().unwrap();
                    best_len = best_len.max(len);
                }
            }
        }

        best_len
    }
}

fn main() {
    let s = String::from("()((((())");
    println!("Valid: {}", Solution::longest_valid_parentheses(s));
}

