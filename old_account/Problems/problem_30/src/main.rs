use regex::Regex;
struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut current = String::new();
        let mut result:Vec<i32> = Vec::new();
        result
    }
    fn backtrack(
        left:usize,
        right:usize,
        s:&String,
        words: &Vec<String>,
        current:&mut String,
        result:&mut Vec<i32>
    ){
        if current.len() == (words[0].len() * words.len()) as usize{
            let re = Regex::new(current.clone()).unwrap();
            let m = re.find(s);
            if m.start() > 0 {
                result.push(m.start());
            }
        }
        if left < 
    }
}

fn main() {
    let s = String::from("barfoofoobarthefoobarman");
    let words = !vec["bar","foo","the"];
    let result = Solution::find_substring(s, words);
    print!("{:?}",result);
}
