use std::io;
//bruh i need some time to understand fck
fn longest_palindrome(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    if n == 0 {
        return String::new();
    }

    let mut start = 0;
    let mut end = 0;

    for i in 0..n {
        // Odd-length palindrome
        let (l1, r1) = expand(&chars, i, i);

        // Even-length palindrome
        let (l2, r2) = expand(&chars, i, i + 1);

        if r1 - l1 > end - start {
            start = l1;
            end = r1;
        }
        if r2 - l2 > end - start {
            start = l2;
            end = r2;
        }
    }

    chars[start..=end].iter().collect()
}

// helper: expand around center
fn expand(chars: &Vec<char>, mut left: usize, mut right: usize) -> (usize, usize) {
    let n = chars.len();

    while right < n && chars[left] == chars[right] {
        if left == 0 {
            break;
        }
        right += 1;
        left -= 1;
    }

    // adjust boundaries
    if chars[left] != chars[right] || right >= n {
        (left + 1, right - 1)
    } else {
        (left, right)
    }
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let res = longest_palindrome(input);
    println!("Longest palindrome: {}", res);
}
