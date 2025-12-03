struct Employee {
    name: String,
    age: u32,
    company: String,
}

fn main() {
    let employee = Employee {
        name: String::from("Alice"),
        age: 30,
        company: String::from("TechCorp"),
    };

    println!("Employee Name: {}", employee.name);
    println!("Employee Age: {}", employee.age);
    println!("Employee Company: {}", employee.company);
}