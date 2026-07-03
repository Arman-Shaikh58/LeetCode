struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let paths: Vec<&str> = path.split("/").collect();
        let mut res: Vec<String> = Vec::new();
        res.push("/".to_string());
        for str in paths {
            if res.is_empty() {
                res.push("/".to_string());
            }
            if str.is_empty() || str == "." {
                continue;
            }
            if str == ".." {
                res.pop();
                res.pop();
                continue;
            }
            res.push(str.to_string());
            res.push("/".to_string());
        }
        res.pop();
        if res.is_empty() {
            "/".to_string()
        } else {
            res.concat()
        }
    }
}

fn main() {
    let path = String::from("/a/../../b/../c//.//");
    println!("Absolute Path: {}", Solution::simplify_path(path));
    // let paths: Vec<&str> = path.split("/").collect();
    // println!("Absolute Path: {:?}", paths);
}
