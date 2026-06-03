use std::collections::HashMap;

struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() {
            return vec![];
        }

        let word_len = words[0].len();
        let word_count = words.len();
        let total_len = word_len * word_count;

        if s.len() < total_len {
            return vec![];
        }

        let mut target: HashMap<&str, i32> = HashMap::new();
        for word in &words {
            *target.entry(word.as_str()).or_insert(0) += 1;
        }

        let mut result = Vec::new();

        for offset in 0..word_len {
            let mut left = offset;
            let mut right = offset;

            let mut current: HashMap<&str, i32> = HashMap::new();
            let mut count = 0;

            while right + word_len <= s.len() {
                let word = &s[right..right + word_len];
                right += word_len;

                if target.contains_key(word) {
                    *current.entry(word).or_insert(0) += 1;
                    count += 1;

                    while current[word] > target[word] {
                        let left_word = &s[left..left + word_len];
                        *current.get_mut(left_word).unwrap() -= 1;
                        left += word_len;
                        count -= 1;
                    }

                    if count == word_count {
                        result.push(left as i32);

                        let left_word = &s[left..left + word_len];
                        *current.get_mut(left_word).unwrap() -= 1;
                        left += word_len;
                        count -= 1;
                    }
                } else {
                    current.clear();
                    count = 0;
                    left = right;
                }
            }
        }

        result
    }
}

fn main() {
    let s = String::from("barfoofoobarthefoobarman");
    let words: Vec<String> = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
    println!("{:?}", Solution::find_substring(s, words));
}
