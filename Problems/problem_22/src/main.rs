
struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result:Vec<String> = Vec::new();
        let mut current = String::new();
        Self::backtrack(n, 0, 0, &mut current, &mut result);
        result
    }
    fn backtrack(
        n: i32,
        open:i32,
        close:i32,
        current:&mut String,
        result:&mut Vec<String>
    ){
        if current.len() == (n*2) as usize{
            result.push(current.clone());
            return;
        }
        if open < n{
            current.push('(');
            Self::backtrack(n, open+1, close, current, result);
            current.pop();
        }
        if close < open{
            current.push(')');
            Self::backtrack(n, open, close+1, current, result);
            current.pop();
        }
    }
}

fn main() {
    let n = 5;
    let result = Solution::generate_parenthesis(n);

    for s in result {
        println!("{}", s);
    }
}
