use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Failed to open file");
    file.write_all("\n Hello class".as_bytes()).expect("Failed to write to file");
    file.write_all("\n This is the appendage to the document".as_bytes()).expect("Failed to write to file");
    println!("file append successful");
}