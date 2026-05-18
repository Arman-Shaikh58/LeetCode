use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_map: HashMap<char, i32> = HashMap::new();
        roman_map.insert('I', 1);
        roman_map.insert('V', 5);
        roman_map.insert('X', 10);
        roman_map.insert('L', 50);
        roman_map.insert('C', 100);
        roman_map.insert('D', 500);
        roman_map.insert('M', 1000);

        let mut number: i32 = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut previous: char;
        for i in 0..s.len() {
            if i != 0 {
                previous = chars[i - 1];

                let pre_val = *roman_map.get(&previous).unwrap_or(&0);
                let cur_val = *roman_map.get(&chars[i]).unwrap_or(&0);
                if pre_val < cur_val {
                    number -= pre_val;
                    number += cur_val - pre_val;
                } else {
                    number += cur_val;
                }
            } else {
                if let Some(val) = roman_map.get(&chars[i]) {
                    number += val;
                }
            }
        }

        number
    }
}

fn main() {
    let s = String::from("MCMXCIV");
    println!("Roman to  i32 is: {}", Solution::roman_to_int(s));
}
