use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    set.insert((i, j));
                }
            }
        }
        let mut filled: HashSet<String> = HashSet::new();

        for t in set {
            let r = format!("r{}", t.0);
            let c = format!("c{}", t.1);

            let mut i = t.0 as i32;

            if !filled.contains(&r) {
                //up
                while i >= 0 {
                    matrix[i as usize][t.1] = 0;
                    i -= 1;
                }
                //down
                i = t.0 as i32;
                while i < m as i32 {
                    matrix[i as usize][t.1] = 0;
                    i += 1;
                }
            }

            if !filled.contains(&c) {
                //left
                i = t.1 as i32;
                while i >= 0 {
                    matrix[t.0][i as usize] = 0;
                    i -= 1;
                }
                //right
                i = t.1 as i32;
                while i < n as i32 {
                    matrix[t.0][i as usize] = 0;
                    i += 1;
                }
            }

            filled.insert(r);
            filled.insert(c);
        }
    }
}

fn main() {
    let mut matrix = vec![vec![1, 1, 2, 2], vec![3, 4, 0, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}
