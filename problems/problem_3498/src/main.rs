struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut res = 0;

        for i in s.as_bytes().iter().enumerate() {
            let reverse_num = 26 - (i.1 - b'a');
            res += reverse_num as i32 * (i.0 as i32 + 1);
        }
        res
    }
}

fn main() {
    let s = String::from("");
    println!("{}", Solution::reverse_degree(s));
}
