use std::io;

struct Solution;
//
// impl Solution {
//     pub fn climb_stairs(n: i32) -> i32 {
//         let mut total_ways = 0;
//         fn backtrack(n: i32, curr_pos: i32, total_ways: &mut i32) {
//             if curr_pos == n {
//                 *total_ways += 1;
//                 return;
//             }
//             if curr_pos > n {
//                 return;
//             }
//             for i in 1..3 {
//                 backtrack(n, curr_pos + i, total_ways);
//             }
//         }
//         backtrack(n, 0, &mut total_ways);
//         total_ways
//     }
// }
//
fn take_input() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Take input");
    input.trim().parse().expect("Failed to parse the number")
}
fn main() {
    println!("Enter a Number");
    let n = take_input();
    println!("The all outcomes are: {}", Solution::climb_stairs(n));
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut a = 1;
        let mut b = 2;

        for _ in 3..=n {
            let c = a + b;
            a = b;
            b = c;
        }

        b
    }
}
