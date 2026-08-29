struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;

        for n in nums {
            twos |= ones & n;

            ones ^= n;

            let threes = ones & twos;

            ones &= !threes;
            twos &= !threes;
        }
        ones
    }
}

fn main() {
    let nums = vec![2, 2, 3, 2];
    println!("{}", Solution::single_number(nums));
}
