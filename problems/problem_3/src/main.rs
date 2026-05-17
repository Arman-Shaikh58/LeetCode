// in this we are gonna solve this problem using sliding window technique

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        let mut longest_substring_len = 0;

        for i in 0..n {
            let mut substring = String::from("");
            for ch in s[i..n].chars() {
                if substring.contains(ch) {
                    break;
                }
                substring.push(ch);
            }
            if longest_substring_len < substring.len() {
                longest_substring_len = substring.len();
            }
        }
        longest_substring_len as i32
    }
}

fn main() {
    let s = String::from("abcabcbb");
    print!(
        "Longest Substring Length: {}",
        Solution::length_of_longest_substring(s)
    );
}
