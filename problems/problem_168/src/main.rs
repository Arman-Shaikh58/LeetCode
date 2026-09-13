struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut res = Vec::new();
        while column_number > 0 {
            let offset = (column_number - 1) % 26;
            res.push((65 + offset as u8) as char);
            column_number = (column_number - 1) / 26;
        }
        res.reverse();
        res.iter().collect()
    }
}

fn main() {
    let num = 721;
    println!("{}", Solution::convert_to_title(num));
}
