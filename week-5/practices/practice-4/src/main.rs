fn main() {
    let full_name = "David Tochukwu Offor";
    let department = "Data Science";
    let uni = "PAU";

    let mut school = "School of Science".to_string();
    // push string
    school.push_str(" and Technology");

    println!("My name is {}", full_name);
    println!("The length of my full name is {}", full_name.len());
    println!("I am a studnet of {}", department);
    println!("{}", school);
    println!("{}", uni);
}
