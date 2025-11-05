use std::io;

fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan Atlantic University";
    let addr:&str = "Km 31 Somewhere in Lagos";
    println!("Name: {}", name);
    println!("University: {} \n Address: {}", uni, addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of science and technology";
    println!("Department: {} \n School: {}", department, school);
}