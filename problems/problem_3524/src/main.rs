struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let n = nums.len();

        let mut result = vec![0; k as usize];
        let mut prev_count = vec![0; k as usize];

        for i in 0..n {
            let mut cur_count = vec![0; k as usize];

            let cur_ele_rem = nums[i] % k;
            cur_count[cur_ele_rem as usize] += 1;

            for old_rem in 0..k {
                let new_rem = (old_rem * (nums[i] % k)) % k;

                cur_count[new_rem as usize] += prev_count[old_rem as usize];
            }
            prev_count = cur_count;
            for x in 0..k {
                result[x as usize] += prev_count[x as usize];
            }
        }
        result
    }
}

fn main() {
    let nums = vec![
        16184625, 380166432, 576506096, 608509523, 171732283, 383734929, 634030393, 16356425,
        409572260, 693752674, 984253617, 395331968, 771764798, 817504354, 537355884, 172144100,
        966212869, 441423288, 30676320, 718483724, 775538272, 966566658, 444298442, 370027368,
        361408129, 290623874, 905666344, 5155089, 595883874, 139854670, 445545098, 710833596,
        831824009, 591341465, 626826620, 404857960, 71243207, 136428386, 374112258, 268026774,
        669207126, 721614497, 186436917, 994218888, 406324719, 633584637, 512372494, 611740006,
        101974906, 313989228, 684914356, 152645531, 991546333, 977887679, 226483852, 292589722,
        671201946, 452151890, 714027210, 291070402, 169296889, 409708413, 816955026, 474809788,
        165315281, 919281926, 32009899, 535302906, 491338136, 862356605, 56606840,
    ];
    let k = 5;
    println!("{:?}", Solution::result_array(nums, k));
}
