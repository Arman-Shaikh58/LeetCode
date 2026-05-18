struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = s.len();
        let mut start: usize = 0;
        let mut max_len: usize = 1;

        for i in 0..len {
            let mut l = i as i32;
            let mut r = i as i32;

            while l >= 0 && r < len as i32 && chars[l as usize] == chars[r as usize] {
                let current_length = r - l + 1;
                if current_length > max_len as i32 {
                    start = l as usize;
                    max_len = current_length as usize;
                }

                l -= 1;
                r += 1;
            }

            let mut l = i as i32;
            let mut r = (i + 1) as i32;

            while l >= 0 && r < len as i32 && chars[l as usize] == chars[r as usize] {
                let current_length = r - l + 1;
                if current_length > max_len as i32 {
                    start = l as usize;
                    max_len = current_length as usize;
                }

                l -= 1;
                r += 1;
            }
        }
        chars[start..start + max_len].iter().collect()
    }
}

fn main() {
    let s = String::from("babad");
    print!("Longest Palifrome: {}", Solution::longest_palindrome(s));
}
