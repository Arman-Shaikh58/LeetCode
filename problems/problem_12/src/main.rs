struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman_list = Vec::from([
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ]);

        let mut res = String::new();
        roman_list.reverse();
        for (value, letter) in roman_list {
            let occ = num / value;
            let letter_values = letter.repeat(occ as usize);
            res.push_str(&letter_values);
            num = num % value;
        }

        res
    }
}

fn main() {
    let num = 3749;
    println!("Roman Representation: {}", Solution::int_to_roman(num));
}
