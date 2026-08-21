struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(vec![1]);

        for i in 1..num_rows {
            let mut cur = Vec::new();
            for j in 0..i + 1 {
                let lt = if i - 1 < 0 || j == 0 {
                    0
                } else {
                    res[(i - 1) as usize][(j - 1) as usize]
                };
                let rt = if j > i || j == i {
                    0
                } else {
                    res[(i - 1) as usize][(j) as usize]
                };
                cur.push(lt + rt);
            }
            res.push(cur);
        }
        res
    }
}

fn main() {
    let nums_rows = 6;
    let sol = Solution::generate(nums_rows);
    for v in sol {
        println!("{:?}", v);
    }
}
