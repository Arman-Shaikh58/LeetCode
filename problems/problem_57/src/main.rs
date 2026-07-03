struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        intervals.sort_unstable_by_key(|v| v[0]);
        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(intervals[0].clone());

        for i in 1..intervals.len() {
            let start = intervals[i][0];
            let end = intervals[i][1];

            let last_ele = res.last_mut().unwrap();

            if last_ele[1] >= start {
                last_ele[1] = last_ele[1].max(end);
            } else {
                res.push(intervals[i].clone());
            }
        }

        res
    }
}

fn main() {
    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval = vec![4, 8];
    println!("{:?}", Solution::insert(intervals, new_interval));
}
