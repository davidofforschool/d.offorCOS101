use std::io;

// i am defining a function 'get coefficients' before the main function. This function will get the coefficients a, b and c easily without having to go through the stress off typing the input functions 3 times

fn get_coefficient(name: &str) -> f64 {
    // im going to loop it indefinitely until the user inputs a valid number
    loop {
        println!("Please enter the coefficient {}:", name);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num) => {
                return num; // if the parsing succeeds
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number."); // if the parsing fails, the user would be prompted to re enter a valid number.
            }
        }
    }
}

fn main() {
    println!("-------- This is a Rust based Quadratic equation solver -------");
    println!("Quadratic equations are in the form: ax^2 + bx + c = 0 \n");

    // were going to get the user input of a, b and c using the get_coefficient function
    let a = get_coefficient("a");
    let b = get_coefficient("b");
    let c = get_coefficient("c");

    println!("The quadratic equation is: {}x² + {}x + {} = 0", a, b, c);

    // now lets calculate the discriminant
    let discriminant = (b.powi(2)) - 4.0 * a * c;
    println!("your discriminant is: {}", discriminant);

    // conditioning for the positivity of the discriminant

    if discriminant > 0.0 {
        // here when the discriminant is greater than 0 (positive), it ahs 2 distinct roots
        let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("\n The equation has two real and distinct roots:");
        println!("x1 = {}", x1);
        println!("x2 = {}", x2);
    }
    else if discriminant == 0.0 {
        // when the discriminant is equal to 0, there is only one real root
        let x = (-b) / (2.0 * a);
        println!("There is only one real root!");
        println!("x = {}", x);
    }
    else {
        // when the discriminant is less than 0, negative, it has 2 complex roots
        // x = (-b ± i * sqrt(-discriminant)) / 2a
        
        let real_part = (-b) / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);

        println!("There are two complex roots: ");
        println!("x1 = {} + {}i", real_part, imaginary_part);
        println!("x2 = {} + {}i", real_part, imaginary_part);
    }
}