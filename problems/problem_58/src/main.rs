struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.trim().split(' ').collect();
        let len = words.len();
        words[len - 1].len() as i32
    }
}

fn main() {
    let s = String::from("luffy is still joyboy");
    println!("Length of last World: {}", Solution::length_of_last_word(s));
}
