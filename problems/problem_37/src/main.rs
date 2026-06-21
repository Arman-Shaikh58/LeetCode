use std::collections::HashSet;
use std::process::Command;
macro_rules! gen_rules {
    ($current:expr,$i:expr,$j:expr) => {
        (
            ($current, 0, $i),
            ($current, 1, $j),
            ($current, 2, ($i / 3) * 3 + $j / 3),
        )
    };
}

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut ruleset: HashSet<(char, usize, usize)> = HashSet::new();

        for i in 0..9 {
            for j in 0..9 {
                let curr = board[i][j];
                if curr != '.' {
                    let (r, c, b) = gen_rules!(curr, i, j);
                    ruleset.insert(r);
                    ruleset.insert(c);
                    ruleset.insert(b);
                }
            }
        }

        fn backtrack(
            board: &mut Vec<Vec<char>>,
            ruleset: &mut HashSet<(char, usize, usize)>,
            i: usize,
            j: usize,
        ) -> bool {
            if i == 9 {
                return true;
            }

            let (ni, nj) = if j == 8 { (i + 1, 0) } else { (i, j + 1) };

            if board[i][j] != '.' {
                return backtrack(board, ruleset, ni, nj);
            }

            for val in 1..=9 {
                let digit = (b'0' + val as u8) as char;
                let (r, c, b) = gen_rules!(digit, i, j);

                if ruleset.contains(&r) || ruleset.contains(&c) || ruleset.contains(&b) {
                    continue;
                }

                board[i][j] = digit;

                ruleset.insert(r);
                ruleset.insert(c);
                ruleset.insert(b);

                // clear_terminal();
                // print_board(board);

                if backtrack(board, ruleset, ni, nj) {
                    return true;
                }

                board[i][j] = '.';

                ruleset.remove(&r);
                ruleset.remove(&c);
                ruleset.remove(&b);

                // clear_terminal();
                // print_board(board);
            }

            false
        }

        backtrack(board, &mut ruleset, 0, 0);
    }
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn print_board(board: &[Vec<char>]) {
    println!("+-------+-------+-------+");

    for (i, row) in board.iter().enumerate() {
        print!("| ");

        for (j, &cell) in row.iter().enumerate() {
            print!("{} ", cell);

            if (j + 1) % 3 == 0 {
                print!("| ");
            }
        }

        println!();

        if (i + 1) % 3 == 0 {
            println!("+-------+-------+-------+");
        }
    }
}

fn main() {
    let mut board: Vec<Vec<char>> = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '9', '.', '.', '1', '.', '.', '3', '.'],
        vec!['.', '.', '6', '.', '2', '.', '7', '.', '.'],
        vec!['.', '.', '.', '3', '.', '4', '.', '.', '.'],
        vec!['2', '1', '.', '.', '.', '.', '.', '9', '8'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '2', '5', '.', '6', '4', '.', '.'],
        vec!['.', '8', '.', '.', '.', '.', '.', '1', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    Solution::solve_sudoku(&mut board);
    println!("{:?}", board);
}
