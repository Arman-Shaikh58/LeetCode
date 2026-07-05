struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut top = 0;
        let mut bottom = m - 1;

        while top <= bottom {
            let mid = top + (bottom - top) / 2;

            if target < matrix[mid][0] {
                if mid == 0 {
                    return false;
                }
                bottom = mid - 1;
            } else if target > matrix[mid][n - 1] {
                top = mid + 1;
            } else {
                let row = mid;

                let mut left = 0;
                let mut right = n - 1;

                while left <= right {
                    let mid = left + (right - left) / 2;

                    if matrix[row][mid] == target {
                        return true;
                    } else if matrix[row][mid] < target {
                        left = mid + 1;
                    } else {
                        if mid == 0 {
                            break;
                        }
                        right = mid - 1;
                    }
                }

                return false;
            }
        }

        false
    }
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;
    print!(
        "Element is Present: {}",
        Solution::search_matrix(matrix, target)
    );
}
