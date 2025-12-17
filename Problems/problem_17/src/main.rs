use std::collections::HashMap;

fn backtracking(
    index: usize,
    digits: &Vec<char>,
    current: &mut String,
    keypad: &HashMap<usize, String>,
    result: &mut Vec<String>,
) {
    if index == digits.len() {
        result.push(current.clone());
        return;
    }

    let digit = digits[index].to_digit(10).unwrap() as usize;

    if let Some(letters) = keypad.get(&digit) {
        for ch in letters.chars() {
            current.push(ch);
            backtracking(index + 1, digits, current, keypad, result);
            current.pop();
        }
    }
}

fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();

    let mut keypad: HashMap<usize, String> = HashMap::new();
    keypad.insert(2, "abc".into());
    keypad.insert(3, "def".into());
    keypad.insert(4, "ghi".into());
    keypad.insert(5, "jkl".into());
    keypad.insert(6, "mno".into());
    keypad.insert(7, "pqrs".into());
    keypad.insert(8, "tuv".into());
    keypad.insert(9, "wxyz".into());

    let mut current = String::new();
    let digits_vec: Vec<char> = digits.chars().collect();

    backtracking(0, &digits_vec, &mut current, &keypad, &mut result);

    result
}

fn main() {
    let digits = String::from("23");
    let combinations = letter_combinations(digits);

    println!("Combinations:");
    for combination in combinations {
        println!("{}", combination);
    }
}
