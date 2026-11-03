struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut column_number: i32 = 0;

        for (i, ch) in column_title.as_bytes().iter().rev().enumerate() {
            let ch_no = (ch - b'A' + 1) as i32;
            column_number += ch_no * 26_i32.pow(i as u32);
        }

        column_number
    }
}

fn main() {
    let column_title = String::from("ZY");
    println!("{}", Solution::title_to_number(column_title));
}
