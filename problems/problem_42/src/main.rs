struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 1 {
            return 0;
        }

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut res = 0;
        let mut max_left = height[left];
        let mut max_right = height[right];

        while left < right {
            if max_left < max_right {
                left += 1;
                max_left = max_left.max(height[left]);
                let cal = max_left - height[left];
                res += if cal <= 0 { 0 } else { cal };
            } else {
                right -= 1;
                max_right = max_right.max(height[right]);
                let cal = max_right - height[right];
                res += if cal <= 0 { 0 } else { cal };
            }
        }
        res
    }
}

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("{}", Solution::trap(height));
}
