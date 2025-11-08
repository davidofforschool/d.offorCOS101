use std::io;

fn main() {
    let p = "Pounded yam and edikaiko soup";
    let f = "fried rice and chicken";
    let a = "amala and ewedu soup";
    let e = "eba and egusi soup";
    let w = "white rice and stew";

    let p_price = 3_200;
    let f_price = 3_000;
    let a_price = 2_500;
    let e_price = 2_000;
    let w_price = 2_500;

    println!("Our menu for today includes:");
    println!("1. {} - ₦{} (type 'p')", p, p_price);
    println!("2. {} - ₦{} (type 'f')", f, f_price);
    println!("3. {} - ₦{} (type 'a')", a, a_price);
    println!("4. {} - ₦{} (type 'e')", e, e_price);
    println!("5. {} - ₦{} (type 'w')", w, w_price);

    let mut user_choice = String::new();
    println!("Input what you want to eat today");
    io::stdin().read_line(&mut user_choice).expect("Failed to read line");
    let user_choice = user_choice.trim().to_lowercase();

    let mut quantity_input = String::new();
    println!("How many plates do you want?");
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");
    let quantity: u32 = quantity_input.trim().parse().expect("Please enter a valid number");

    // conditioning for different foods [basically, we want to do different calculations based on what the user chose]

    let total_cost:f64 = match user_choice.as_str() {
        "p" => (p_price * quantity).into(),
        "f" => (f_price * quantity).into(),
        "a" => (a_price * quantity).into(),
        "e" => (e_price * quantity).into(),
        "w" => (w_price * quantity).into(),
        _ => {
            println!("Invalid choice. Please select a valid food item.");
            return;
        }
    };

    println!("Your total cost is: ₦{}", total_cost);

    if total_cost > 10_000.0 {
        println!("You qualify for a 5% discount!");
        let discount:f64 = 0.05 * (total_cost);
        let discounted_total = total_cost - discount;
        println!("Your total cost after discount is: ₦{}", discounted_total);
        return;
    }

    println!("Thank you for dining with us!");

    let mut restart_choice = String::new();
    println!("Do you want to place another order? (yes/no)");
    io::stdin().read_line(&mut restart_choice).expect("Failed to read line");
    let restart_choice = restart_choice.trim().to_lowercase();

    if restart_choice == "yes" {
        main();
    } else {
        println!("Goodbye!");
    }
}