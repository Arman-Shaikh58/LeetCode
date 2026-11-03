struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut total_moves = 0;
        let mut largest_element = i32::MIN;

        for ele in &nums{
            if *ele > largest_element{
                largest_element=*ele;
            }
        }

        for ele in &nums{
            total_moves += largest_element - *ele;
        }
        total_moves
    }
}

fn main(){
    let nums = vec![2,1,3];
    println!("{}",Solution::min_moves(nums));
}
