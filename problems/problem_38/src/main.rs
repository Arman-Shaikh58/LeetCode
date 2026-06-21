use std::io;

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        fn backtrack(n: i32) -> String {
            if n <= 1 {
                return "1".to_string();
            }

            let res: Vec<char> = backtrack(n - 1).chars().collect();
            let mut rle = String::new();
            let len = res.len();
            let mut i = 0;
            while i < len {
                let mut count = 1;
                while i < len - 1 && res[i] == res[i + 1] {
                    count += 1;
                    i += 1;
                }
                rle.push_str(count.to_string().as_str());
                rle.push(res[i]);
                i += 1;
            }
            println!("{rle}");
            rle.chars().collect()
        }
        backtrack(n)
    }
}
fn take_input() -> i32 {
    println!("Enter a Number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = take_input();
    println!("{}", Solution::count_and_say(n));
}
