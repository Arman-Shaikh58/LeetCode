struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|i| i[0]);
        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(intervals[0].clone());

        for i in 1..intervals.len() {
            let start = intervals[i][0];
            let end = intervals[i][1];

            let last = res.last_mut().unwrap();

            if start <= last[1] {
                last[1] = last[1].max(end);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res
    }
}

fn main() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    println!("{:?}", Solution::merge(intervals));
}
