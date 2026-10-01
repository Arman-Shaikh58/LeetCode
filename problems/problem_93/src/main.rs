struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        if s.len() < 4 || s.len() > 12 {
            return res;
        }

        fn backtrack(
            s: &String,
            dots: usize,
            i: usize,
            current: &mut String,
            res: &mut Vec<String>,
        ) {
            if dots == 4 {
                if i == s.len() {
                    res.push(current.clone());
                }
                return;
            }
            let remaining = 4 - dots;
            let chars_left = s.len() - i;
            if chars_left < remaining || chars_left > remaining * 3 {
                return;
            }

            for j in i..(s.len().min(i + 3)) {
                let addr = &s[i..=j];
                if addr.len() > 1 && addr.starts_with('0') {
                    break;
                }
                let num: u16 = addr.parse().unwrap();

                if num > 255 {
                    break;
                }

                let prev_len = current.len();

                if dots > 0 {
                    current.push('.');
                }
                current.push_str(addr);

                backtrack(s, dots + 1, j + 1, current, res);

                current.truncate(prev_len);
            }
        }

        let mut current = String::new();
        backtrack(&s, 0, 0, &mut current, &mut res);

        res
    }
}

fn main() {
    let s = String::from("101023");
    print!("{:?}", Solution::restore_ip_addresses(s));
}
