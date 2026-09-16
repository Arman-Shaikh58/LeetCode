struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s = s.as_bytes();
        let mut count = 0;

        let is_palidrome = |slice: &[u8]| slice.iter().eq(slice.iter().rev());

        let mut i = 0;
        while i + k <= s.len() {
            if is_palidrome(&s[i..i + k]) {
                count += 1;
                i += k;
            } else if i + k < s.len() && is_palidrome(&s[i..i + k + 1]) {
                count += 1;
                i += k + 1;
            } else {
                i += 1;
            }
        }
        count
    }
}

fn main() {
    let s = String::from("waxrmmrxawehtdujvgvjudmpyjozoxozojilihjlyw");
    let k = 3;
    println!("Solution {}", Solution::max_palindromes(s, k));
}
