struct Employee {
    ceo: String,
    age: u32,
    company: String
}

fn main() {
    let emp1 = Employee {
        ceo: String::from("Satya Nadella"),
        age: 45,
        company: String::from("Microsoft")
    };

    let emp2 = Employee {
        ceo: String::from("Sundar Pichai"),
        age: 50,
        company: String::from("Google")
    };

    display(emp1);
    display(emp2);
}

fn display(emp: Employee) {
    println!("CEO: {}", emp.ceo);
    println!("Age: {}", emp.age);
    println!("Company: {}", emp.company);
    println!("---------------------");
}