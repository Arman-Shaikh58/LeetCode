struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left = 0;
        let mut right = n - 1;
        let mut water_max_area = 0;
        while left < right {
            let new_area;
            if height[left] < height[right] {
                new_area = height[left] * (right - left) as i32;
            } else {
                new_area = height[right] * (right - left) as i32;
            }
            if new_area > water_max_area {
                water_max_area = new_area;
            }
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        water_max_area
    }
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("Max Water Area: {}", Solution::max_area(height));
}
