struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut tank = 0;
        let mut start = 0;

        for i in 0..gas.len() {
            let diff = gas[i] - cost[i];
            total += diff;
            tank += diff;

            if tank < 0 {
                start = i + 1;
                tank = 0;
            }
        }

        if total < 0 { -1 } else { start as i32 }
    }
}

fn main() {
    let gas = vec![5, 1, 2, 3, 4];
    let cost = vec![4, 4, 1, 5, 1];
    println!("{}", Solution::can_complete_circuit(gas, cost));
}
