struct Solution;

impl Solution {
    pub fn merge_sort(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums1.len();
        let m = nums2.len();

        let mut x: usize = 0;
        let mut y: usize = 0;

        let mut z: Vec<i32> = Vec::new();

        while x < n && y < m {
            if nums1[x] < nums2[y] {
                z.push(nums1[x].clone());
                x += 1;
            } else {
                z.push(nums2[y].clone());
                y += 1;
            }
        }
        while x < n {
            z.push(nums1[x].clone());
            x += 1;
        }
        while y < m {
            z.push(nums2[y].clone());
            y += 1;
        }
        // returned merged sorted array
        z
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len();
        let m = nums2.len();

        let z = Solution::merge_sort(nums1, nums2);
        let total_size = m + n;

        if total_size % 2 == 0 {
            (z[total_size / 2] as f64 + z[(total_size - 1) / 2] as f64) / 2 as f64
            // i got a test case wrong here because index of an array starts with zero not 1 so
            // it should be n-1 not n+1
            // (n/2 + (n+1)/2 )/2 = median for even number of elements
        } else {
            z[total_size / 2] as f64
        }
    }
}

fn main() {
    let nums1 = vec![9, 19, 29, 39, 49, 59, 69, 79];
    let nums2 = vec![7, 14, 21, 28, 35, 42, 49];

    let median = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("The median is: {median}");
}
