use std::io;

fn value(n:Option<&char>) {
    println!("Element of vector {:?}", n);
}

fn main() {
    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N'];

    let mut input1 = String::new();
    println!("\n Enter an index value between 0 to 8: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    let index: usize = input1.trim().parse().expect("Please enter a valid number");

    let ch:Option<&char> = v.get(index);
    value(ch);
}