struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut lenght_longest_substring = 0;

        let mut window = String::from("");
        for ch in s.chars() {
            if !window.contains(ch) {
                window.push(ch);
                if lenght_longest_substring < window.len() {
                    lenght_longest_substring = window.len();
                }
            } else {
                window.remove(0);
            }
        }

        lenght_longest_substring as i32
    }
}

fn main() {
    let s = String::from("abcabcbb");
    println!(
        "Longet Substring length : {}",
        Solution::length_of_longest_substring(s)
    );
}
