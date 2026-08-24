use std::io;
struct Solution;

// How to generate Gray code?
// The prefix and reflect method are recursively used to generate the Gray code
// of a number. For generating gray code:
// 1. We find the number of bits required to represent a number.
// 2. Next, we find the code for 0, i.e., 0000, which is the same as binary.
// 3. Now, we take the previous code, i.e., 0000, and change the most
// significant bit of it.
// 4. We perform this process reclusively until all the codes are not uniquely
// identified.
// 5. If by changing the most significant bit, we find the same code obtained
// previously, then the second most significant bit will be changed, and so
// on.

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let size = 1 << n;
        let mut res: Vec<i32> = Vec::with_capacity(size as usize);
        for i in 0..size {
            res.push(i ^ (i >> 1));
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{:?}", Solution::gray_code(input.trim().parse().unwrap()));
}
