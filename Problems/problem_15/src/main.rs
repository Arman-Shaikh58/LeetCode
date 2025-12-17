struct Solution;

impl Solution {
    pub fn bubble_sort(nums: &mut Vec<i32>) {
        let len = nums.len();
        for i in 0..len {
            for j in i + 1..len {
                if nums[i] > nums[j] {
                    nums.swap(i, j);
                }
            }
        }
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let len = nums.len();

        if len < 3 {
            return result;
        }

        Solution::bubble_sort(&mut nums);

        for i in 0..len - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    let output = Solution::three_sum(nums);
    println!("{:?}", output);
}


// the code below is from someone in leetcode


// impl Solution {
//     pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
//         // Sort the array to enable two-pointer technique and duplicate skipping
//         nums.sort_unstable();
//         let array_length = nums.len();
//         let mut triplet_results = vec![];

//         // Iterate through each number as the first element of potential triplet
//         for first_index in 0..array_length.saturating_sub(2) {
//             let first_number = nums[first_index];

//             // Optimization: if first number is positive, no way to sum to zero
//             // (all remaining numbers are also positive or zero)
//             if first_number > 0 {
//                 break;
//             }

//             // Skip duplicates for the first position to avoid duplicate triplets
//             if first_index > 0 && first_number == nums[first_index - 1] {
//                 continue;
//             }

//             // Early termination: if smallest 3 numbers from here exceed 0, stop
//             if first_number + nums[first_index + 1] + nums[first_index + 2] > 0 {
//                 break;
//             }

//             // Skip: if largest 2 numbers + current can't reach 0, try next first
//             if first_number + nums[array_length - 2] + nums[array_length - 1] < 0 {
//                 continue;
//             }

//             // Two-pointer approach: start from both ends of remaining array
//             let (mut left_pointer, mut right_pointer) = (first_index + 1, array_length - 1);

//             while left_pointer < right_pointer {
//                 let current_sum = first_number + nums[left_pointer] + nums[right_pointer];

//                 if current_sum < 0 {
//                     // Sum too small, move left pointer right to increase sum
//                     left_pointer += 1;
//                 } else if current_sum > 0 {
//                     // Sum too large, move right pointer left to decrease sum
//                     right_pointer -= 1;
//                 } else {
//                     // Found a valid triplet
//                     triplet_results.push(vec![
//                         first_number,
//                         nums[left_pointer],
//                         nums[right_pointer],
//                     ]);

//                     // Move both pointers and skip duplicates
//                     left_pointer += 1;
//                     right_pointer -= 1;

//                     // Skip duplicate values for left pointer
//                     while left_pointer < right_pointer
//                         && nums[left_pointer] == nums[left_pointer - 1]
//                     {
//                         left_pointer += 1;
//                     }

//                     // Skip duplicate values for right pointer
//                     while left_pointer < right_pointer
//                         && nums[right_pointer] == nums[right_pointer + 1]
//                     {
//                         right_pointer -= 1;
//                     }
//                 }
//             }
//         }

//         triplet_results
//     }
// }

// i think it submitted by someone using gpt