use std::io;

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().parse::<f64>().expect("Please enter a valid number")
}

fn area_trapezium() -> f64 {
    let height = read_input("Enter the height:");
    let base1 = read_input("Enter base 1:");
    let base2 = read_input("Enter base 2:");

    (height / 2.0) * (base1 + base2)
}

fn area_rhombus() -> f64 {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");

    0.5 * d1 * d2
}


fn area_parallelogram() -> f64 {
    let base = read_input("Enter the base:");
    let altitude = read_input("Enter the altitude:");

    base * altitude
}


fn area_cube() -> f64 {
    let side = read_input("Enter the side length:");
    6.0 * side * side
}


fn volume_cylinder() -> f64 {
    let radius = read_input("Enter the radius:");
    let height = read_input("Enter the height:");

    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("Select a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = choice.trim().parse().expect("Enter a valid number");

    let result = match choice {
        1 => area_trapezium(),
        2 => area_rhombus(),
        3 => area_parallelogram(),
        4 => area_cube(),
        5 => volume_cylinder(),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    println!("Result = {}", result);
}