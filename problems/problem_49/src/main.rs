use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut count = [0u8; 26];

            for b in s.as_bytes() {
                count[(b - b'a') as usize] += 1;
            }
            group.entry(count).or_default().push(s);
        }
        group
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<Vec<String>>>() // i dont know
        // how but  this above line gives 0ms but below gives 4ms
        //group.into_values().collect()
    }
}

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    println!("{:?}", Solution::group_anagrams(strs));
}
