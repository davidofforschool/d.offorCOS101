use std::io;

fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;

    println!("the sum of a and b is: {}", sum);
}

fn main() {
    let mut input1 = String::new();
    println!("Enter the parameter for A: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let a: i32 = input1.trim().parse().expect("Please type a number!");

    let mut input2 = String::new();
    println!("Enter the parameter for B: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let b: i32 = input2.trim().parse().expect("Please type a number!");

    add(a, b);
}