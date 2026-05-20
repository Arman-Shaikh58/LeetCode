use std::io;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn is_open_bracket(ch: char) -> bool {
            let open_brackets = String::from("{[(");
            open_brackets.contains(ch)
        }

        fn check_valid_b(open_br: char, closed_br: char) -> bool {
            open_br == '(' && closed_br == ')'
                || open_br == '[' && closed_br == ']'
                || open_br == '{' && closed_br == '}'
        }

        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            if is_open_bracket(ch) {
                stack.push(ch);
            } else {
                let last_char = match stack.pop() {
                    Some(val) => val,
                    None => return false,
                };

                if !check_valid_b(last_char, ch) {
                    return false;
                }
            }
        }
        if !stack.is_empty() {
            return false;
        }
        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");
    println!(
        "Is Valid : {}",
        Solution::is_valid(input.trim().to_string())
    );
}
