
fn max_area(height: Vec<i32>) -> i32 {
        let mut left_pointer=0;
        let mut right_pointer = height.len()-1;
        let mut max_capacity  = 0;

        while left_pointer<right_pointer {
            let smaller_wall;
            let current_capacity;
            if height[left_pointer]<height[right_pointer] {
                smaller_wall = height[left_pointer];
                current_capacity = smaller_wall * (right_pointer-left_pointer) as i32;
                left_pointer+=1;
            }else {
                smaller_wall = height[right_pointer];
                current_capacity = smaller_wall * (right_pointer-left_pointer) as i32;
                right_pointer-=1;
            }
            if current_capacity>max_capacity {
            max_capacity=current_capacity;
            }
        }
        max_capacity
    }

fn main(){
    let heights = vec![1,8,6,2,5,4,8,3,7];
    println!("Output: {}",max_area(heights));
}