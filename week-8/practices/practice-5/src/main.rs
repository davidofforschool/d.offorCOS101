use std::io;

fn main() {
    let mut city : Vec<String> = Vec::new();

    println!("The city vector has element {}", city.len());

    let mut input1 = String::new();
    println!("How mant cities do you want to enter?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let city_num:i32 = input1.trim().parse().expect("Please type a number!");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter city name: {}", count + 1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read line");
        let new_city = input2.trim().to_string();
        city.push(new_city);
    }
    println!("Your preffered cities are:");
    let mut count = 1;

    for i in city {
        println!("{}. {}", count, i);
        count += 1;
    }
}