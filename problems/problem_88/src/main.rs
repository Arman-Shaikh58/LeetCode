struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let n1 = nums1.clone();
        let m: usize = n1.len() - n as usize;
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut z = 0;
        while i < m && j < n as usize {
            if n1[i] < nums2[j] {
                nums1[z] = n1[i];
                i += 1;
                z += 1;
            } else {
                nums1[z] = nums2[j];
                j += 1;
                z += 1;
            }
            println!("{:?}", nums1);
        }

        while i < m {
            nums1[z] = n1[i];
            i += 1;
            z += 1;
            println!("{:?}", nums1);
        }
        while j < n as usize {
            nums1[z] = nums2[j];
            j += 1;
            z += 1;
            println!("{:?}", nums1);
        }

        println!("{:?}", n1);
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);
}
