use std::io;

fn main() {
    let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];
    let mut input1 = String::new();

    println!("Enter the first index (0-7): ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let index:usize = input1.trim().parse().expect("Please enter a valid number");

    let ch: char = v[index];
    println!("{} is the character for the index [{}]\n", ch, index);
}
