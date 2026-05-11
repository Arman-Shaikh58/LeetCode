struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut final_prefix = String::from("");
        let n = strs.len();
        let length_of_first_word = strs[0].len();

        for i in 0..length_of_first_word {
            let first_word: &str = &strs[0];
            let mut prefix = final_prefix.clone();
            prefix.push_str(&first_word[i..i + 1]);
            let mut contains = true;
            for j in 0..n {
                if !strs[j].starts_with(&prefix) {
                    contains = false;
                    break;
                }
            }
            if !contains {
                break;
            } else {
                final_prefix = prefix;
            }
        }
        final_prefix
    }
}

fn main() {
    let input: Vec<String> = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!(
        "Longest Common Prefix: {}",
        Solution::longest_common_prefix(input)
    );
}
