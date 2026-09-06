use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut cache: HashMap<(usize, usize), i32> = HashMap::new();

        fn backtrack(
            s_idx: usize,
            t_idx: usize,
            s: &Vec<char>,
            t: &Vec<char>,
            cache: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if t_idx == t.len() {
                return 1;
            }
            if s_idx >= s.len() {
                return 0;
            }

            if cache.contains_key(&(s_idx, t_idx)) {
                return *cache.get(&(s_idx, t_idx)).unwrap();
            }

            if s[s_idx] == t[t_idx] {
                let otc = backtrack(s_idx + 1, t_idx + 1, s, t, cache)
                    + backtrack(s_idx + 1, t_idx, s, t, cache);
                cache.insert((s_idx, t_idx), otc);
            } else {
                let otc = backtrack(s_idx + 1, t_idx, s, t, cache);
                cache.insert((s_idx, t_idx), otc);
            }
            *cache.get(&(s_idx, t_idx)).unwrap()
        }
        backtrack(0, 0, &s, &t, &mut cache)
    }
}

fn main() {
    let s = String::from("babgbag");

    let t = String::from("bag");

    println!("{}", Solution::num_distinct(s, t));
}
