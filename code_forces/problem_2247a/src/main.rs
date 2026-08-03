use std::io;

fn take_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");
    input.trim().to_string()
}

fn main() {
    let no_of_test_cases = take_input().parse().unwrap();
    for _ in 0..no_of_test_cases {
        let array_length: usize = take_input().parse().unwrap();
        let mut sum = 0;
        for num in take_input().split(" ") {
            let n: i32 = num.parse().unwrap();
            sum += n;
        }

        if array_length % 2 == 0 && sum % 4 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
