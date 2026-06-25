use std::io;

struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; n as usize]; n as usize];

        let mut count = 1;
        let mut left = 0;
        let mut right = n;
        let mut top = 0;
        let mut bottom = n;

        while left < right && top < bottom {
            //left to right
            for i in left..right {
                res[top as usize][i as usize] = count;
                count += 1;
            }
            top += 1;
            //top to bottom
            for i in top..bottom {
                res[i as usize][right as usize - 1] = count;
                count += 1;
            }
            right -= 1;

            if !(left < right && top < bottom) {
                break;
            }

            //right to left

            for i in (left..right).rev() {
                res[bottom as usize - 1][i as usize] = count;
                count += 1;
            }
            bottom -= 1;

            //bottom to top
            for i in (top..bottom).rev() {
                res[i as usize][left as usize] = count;
                count += 1;
            }
            left += 1;
        }
        res
    }
}
fn take_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = take_input();
    let mat = Solution::generate_matrix(n);
    for i in 0..n {
        for j in 0..n {
            print!("{} ", mat[i as usize][j as usize]);
        }
        print!("\n");
    }
}
