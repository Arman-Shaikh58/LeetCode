struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut dp2 = 1; // dp[i + 2]
        let mut dp1 = if bytes[n - 1] != b'0' { 1 } else { 0 }; // dp[i + 1]

        if n == 1 {
            return dp1;
        }

        for i in (0..n - 1).rev() {
            let mut curr = 0;

            if bytes[i] != b'0' {
                curr += dp1;

                let two_digit = (bytes[i] - b'0') as i32 * 10 + (bytes[i + 1] - b'0') as i32;

                if two_digit >= 10 && two_digit <= 26 {
                    curr += dp2;
                }
            }

            dp2 = dp1;
            dp1 = curr;
        }

        dp1
    }
}

fn main() {
    let s = String::from("12312");
    println!("{}", Solution::num_decodings(s));
}
