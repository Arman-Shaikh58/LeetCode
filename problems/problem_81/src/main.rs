struct Solution;

// impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> bool {
//         let mut left = 0;
//         let mut right = nums.len() - 1;
//         while left <= right {
//             let mid = left + (right - left) / 2;
//
//             if nums[mid] == target {
//                 return true;
//             }
//
//             if nums[left] < nums[right] {
//                 //left porttion of the array
//
//                 if nums[left] <= target && target < nums[mid] {
//                     right = mid - 1;
//                 } else {
//                     left = mid + 1;
//                 }
//             } else if nums[left] > nums[mid] {
//                 // right porttion of the array
//                 if nums[left] < target && target < nums[mid] {
//                     left = mid + 1;
//                 } else {
//                     right = mid - 1;
//                 }
//             } else {
//                 left += 1;
//             }
//         }
//         false
//     }
// }

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return true;
            }

            // which portion is sorted
            if nums[left] == nums[mid] {
                left += 1;
                continue;
            }

            // left side of the array
            if nums[left] < nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            // right side of the array
            else {
                if nums[mid] < target && target <= nums[right - 1] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }

        false
    }
}

fn main() {
    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let target = 0;
    println!("{}", Solution::search(nums, target));
}
