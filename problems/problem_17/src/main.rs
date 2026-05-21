use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut letters: HashMap<char, &str> = HashMap::new();

        letters.insert('2', "abc");
        letters.insert('3', "def");
        letters.insert('4', "ghi");
        letters.insert('5', "jkl");
        letters.insert('6', "mno");
        letters.insert('7', "pqrs");
        letters.insert('8', "tuv");
        letters.insert('9', "wxyz");

        let mut res: Vec<String> = Vec::new();
        let mut cur_str = String::new();

        fn backtrack(
            i: usize,
            cur_str: &mut String,
            digits: &String,
            letters: &HashMap<char, &str>,
            res: &mut Vec<String>,
        ) {
            if i == digits.len() {
                res.push(cur_str.clone());
                return;
            }

            let digit = digits.chars().nth(i).unwrap();

            if let Some(chars) = letters.get(&digit) {
                for ch in chars.chars() {
                    cur_str.push(ch);

                    backtrack(i + 1, cur_str, digits, letters, res);

                    cur_str.pop();
                }
            }
        }

        backtrack(0, &mut cur_str, &digits, &letters, &mut res);

        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Faield to take inpt");
    print!("all combiantions : ");
    for ch in Solution::letter_combinations(input.trim().to_string()) {
        print!("{ch} ");
    }
}
