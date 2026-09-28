struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let s = s.as_bytes();
        let is_parentheses = |ch: &u8| -> bool { !"1234567890+-*/".contains(*ch as char) };
        let mut curr = 0;
        let mut max = 0;
        for ch in s {
            if !is_parentheses(ch) {
                continue;
            }
            if *ch == '(' as u8 {
                curr += 1;
            } else {
                curr -= 1;
            }
            if curr > max {
                max = curr;
            }
        }
        max
    }
}
fn main() {
    let s = String::from("(1)+((2))+(((3)(())))");
    println!("Solution: {}", Solution::max_depth(s));
}
