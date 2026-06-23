struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();

        let mut left = 0usize;
        let mut right = matrix[0].len();
        let mut top = 0usize;
        let mut bottom = matrix.len();

        while left < right && top < bottom {
            //left to right
            for i in left..right {
                res.push(matrix[top][i]);
            }
            top += 1;

            //right to bottom
            for i in top..bottom {
                res.push(matrix[i][right - 1]);
            }
            right -= 1;

            if !(top < bottom && left < right) {
                break;
            }

            for i in (left..right).rev() {
                res.push(matrix[bottom - 1][i]);
            }
            bottom -= 1;

            for i in (top..bottom).rev() {
                res.push(matrix[i][left]);
            }
            left += 1;
        }

        res
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    println!("{:?}", Solution::spiral_order(matrix));
    println!("[1,2,3,4,8,12,11,10,9,5,6,7]");
}
