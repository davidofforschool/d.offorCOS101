fn checker() {
    let mut input = String::new();
    println!("enter a character: ");
    std::io::stdin().read_line(&mut input).expect("failed to read line");
    let ch:char = input.trim().parse().expect("please enter a character");

    if ch >= '0' && ch <= '9' {
        println!("Character '{}' is a digit", ch);
    } 
    else {
        println!("Character '{}' is a special character", ch);
    }
}

fn main() {
    println!("Welcome! this program checks if a character is a digit or a special character.");
    checker();
}