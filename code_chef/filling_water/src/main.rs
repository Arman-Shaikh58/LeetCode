use std::io;

fn take_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let no_of_tc = take_input().parse().unwrap();
    for _ in 0..no_of_tc {
        let mut sum: i32 = 0;
        for ele in take_input().split(' ') {
            sum += ele.trim().parse::<i32>().unwrap();
        }
        if sum < 2 {
            println!("Water filling time");
        } else {
            println!("Not now");
        }
    }
}
