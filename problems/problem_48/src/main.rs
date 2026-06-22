struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut l = 0;
        let mut r = matrix.len() - 1;
        while l < r {
            for i in 0..r - l {
                let top = l;
                let bottom = r;

                //place top let element in the temp var
                let temp = matrix[top][l + i];

                // move bottom left to top left
                matrix[top][l + i] = matrix[bottom - i][l];

                // move bottom right to the bottom left
                matrix[bottom - i][l] = matrix[bottom][r - i];

                // move top right to the bottom right
                matrix[bottom][r - i] = matrix[top + i][r];

                matrix[top + i][r] = temp;
            }
            l += 1;
            r -= 1;
        }
    }
}

fn main() {
    let mut matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    println!("{:?}", matrix);
    println!("{:?}", Solution::rotate(&mut matrix));
}
