use std::io;

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(vec![1]);

        for i in 1..row_index + 1 {
            let mut cur = Vec::new();
            for j in 0..i + 1 {
                let lt = if i - 1 < 0 || j == 0 {
                    0
                } else {
                    res[(i - 1) as usize][(j - 1) as usize]
                };
                let rt = if j > i || j == i {
                    0
                } else {
                    res[(i - 1) as usize][(j) as usize]
                };
                cur.push(lt + rt);
            }
            res.push(cur);
        }
        res.pop().unwrap()
    }
}

fn main() {
    println!("Enter number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{:?}", Solution::get_row(input.trim().parse().unwrap()));
}
