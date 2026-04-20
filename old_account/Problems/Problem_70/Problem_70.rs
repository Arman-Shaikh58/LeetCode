use std::io;

fn main(){
    println!("Hello World");//hehe just for fun
    println!("Enter a Number");
    let mut input= String::new();//string because when using String instead of str becusing String is used when we are assigning value at runtime not at the compile time,
                                //  str is used when string is giving at the time of writing code
    io::stdin().read_line(&mut input).expect("Failed to read Input");

    let n = input.trim().parse().expect("Failed to parse the number");

    let mut step1=1;
    let mut step2=0;
    let mut total_ways=0;
    for _ in 0..n{
        total_ways=step1+step2;
        step2=step1;
        step1=total_ways;
    }

    println!("The Total ways are: {total_ways}");


}

// man i forgot the syntax for for loop and taking input 🥲