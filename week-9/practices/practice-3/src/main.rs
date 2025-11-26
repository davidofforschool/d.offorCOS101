use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("Failed to delete the file");
    println!("File deleted successfully.");
}