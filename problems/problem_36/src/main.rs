use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut ruleset: HashSet<String> = HashSet::new();
        for i in 0..9 {
            for j in 0..9 {
                let current = board[i][j];
                if current == '.' {
                    continue;
                }
                let row = format!("{}_is_row_{}", current, j);
                let column = format!("{}_is_column_{}", current, i);
                let boxi: String = format!("{}_is_box_{}_{}", current, i / 3, j / 3);
                if ruleset.contains(&row) || ruleset.contains(&column) || ruleset.contains(&boxi) {
                    return false;
                }
                ruleset.insert(row);
                ruleset.insert(boxi);
                ruleset.insert(column);
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
