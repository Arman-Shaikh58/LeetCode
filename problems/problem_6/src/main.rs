struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() == 1 || num_rows == 1 {
            return s;
        }
        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];

        let mut down = false;
        let mut row: usize = 0;
        for ch in s.chars() {
            rows[row].push(ch);

            if row == 0 || row == (num_rows - 1) as usize {
                down = !down;
            }

            if !down {
                row -= 1;
            } else {
                row += 1;
            }
        }
        rows.concat()
    }
}

fn main() {
    let s = String::from("PAYPALISHIRING");
    println!("Cipher Text: {}", Solution::convert(s.clone(), 3));
    assert_eq!("PAHNAPLSIIGYIR", Solution::convert(s, 3));
}
