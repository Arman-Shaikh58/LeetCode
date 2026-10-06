struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut closed_needed = 0;
        let mut balanced = 0;

        for ch in s.chars() {
            if ch == '(' {
                balanced += 1;
            } else {
                if balanced == 0 {
                    closed_needed += 1;
                } else {
                    balanced -= 1;
                }
            }
        }
        balanced + closed_needed
    }
}

fn main() {
    let s = String::from("(())))");
    println!("{}", Solution::min_add_to_make_valid(s));
}
