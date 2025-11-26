use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input & Output \n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("Could not create file");
    file.write_all("Welcome to Rust programming!\n"
        .as_bytes()).expect("Could not write to file");
    file.write_all(announce.as_bytes()).expect("Could not write to file");
    file.write_all(dept.as_bytes()).expect("Could not write to file");
    println!("\n Data written to file successfully.");
}