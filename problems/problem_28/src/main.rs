use std::io;

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if !haystack.contains(&needle) {
            return -1;
        }
        let needle_chars: Vec<char> = needle.chars().collect();
        let needle_len = needle.len();
        for (i, ch) in haystack.chars().enumerate() {
            if ch == needle_chars[0] {
                if &haystack[i..i + needle_len] == needle {
                    return i as i32;
                }
            }
        }
        -1
    }
}

fn take_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to take input");
    input.trim().to_string()
}

fn main() {
    println!("Enter the haystack");
    let haystack = take_input();
    println!("Enter the needle:");
    let needle = take_input();

    println!(
        "Index of needle is: {}",
        Solution::str_str(haystack, needle)
    );
}
