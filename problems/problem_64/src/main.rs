struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut mat = vec![i32::MAX; n];
        for i in (0..m).rev() {
            let mut new_mat = vec![i32::MAX; n];
            for j in (0..n).rev() {
                if i == m - 1 && j == n - 1 {
                    new_mat[j] = grid[i][j];
                } else {
                    let right = if j + 1 < n { new_mat[j + 1] } else { i32::MAX };
                    let down = mat[j];
                    new_mat[j] = grid[i][j] + right.min(down);
                }
            }
            mat = new_mat;
        }
        mat[0]
    }
}

fn main() {
    // let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println!("{}", Solution::min_path_sum(grid));
}
