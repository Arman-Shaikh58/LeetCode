struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 0;
        digits.reverse();
        let len = digits.len();
        for i in 0..len {
            if i == 0 {
                let sum = digits[i] + 1;
                digits[i] = sum % 10;
                carry = sum / 10;
            } else {
                let sum = digits[i] + carry;
                digits[i] = sum % 10;
                carry = sum / 10;
            }
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            digits.push(carry);
        }
        digits.reverse();
        digits
    }
}

fn main() {
    let digits = vec![9, 9, 9, 9];
    println!("{:?}", Solution::plus_one(digits));
}
