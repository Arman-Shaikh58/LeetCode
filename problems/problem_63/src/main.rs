struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        if obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }
        obstacle_grid[m - 1][n - 1] = -1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if obstacle_grid[i][j] == 1 {
                    obstacle_grid[i][j] = 0;
                    continue;
                }
                if j == n - 1 && i == m - 1 {
                    continue;
                }
                obstacle_grid[i][j] = if j >= n - 1 {
                    0
                } else {
                    obstacle_grid[i][j + 1]
                } + if i >= m - 1 {
                    0
                } else {
                    obstacle_grid[i + 1][j]
                };
            }
        }
        println!("{:?}", obstacle_grid);

        obstacle_grid[0][0].abs()
    }
}

fn main() {
    let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    println!(
        "All Possible Solutions : {}",
        Solution::unique_paths_with_obstacles(obstacle_grid)
    );
}
