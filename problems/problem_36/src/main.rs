use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // horizontally
        for row in 0..9 {
            let mut seen = HashSet::new();

            for col in 0..9 {
                let ch = board[row][col];

                if ch == '.' {
                    continue;
                }

                if !seen.insert(ch) {
                    return false;
                }
            }
        }

        // verticvallay
        for col in 0..9 {
            let mut seen = HashSet::new();

            for row in 0..9 {
                let ch = board[row][col];

                if ch == '.' {
                    continue;
                }

                if !seen.insert(ch) {
                    return false;
                }
            }
        }

        // check 3 by 3 box
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                let mut seen = HashSet::new();

                for row in box_row..box_row + 3 {
                    for col in box_col..box_col + 3 {
                        let ch = board[row][col];

                        if ch == '.' {
                            continue;
                        }

                        if !seen.insert(ch) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("Board is: {}", Solution::is_valid_sudoku(board));
}
