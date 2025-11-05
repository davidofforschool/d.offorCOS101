use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Please enter the coefficient a:");
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Please enter a valid number");    

    println!("Please enter the coefficient b:");
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Please enter a valid number");

    println!("Please enter the coefficient c:");
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: f64 = c.trim().parse().expect("Please enter a valid number");

    println!("The quadratic equation is: {}x² + {}x + {} = 0", a, b, c);
}

kjzkgp