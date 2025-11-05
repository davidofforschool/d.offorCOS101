use std::io;

fn main() {
    let mut p = "Pounded yam and edikaiko soup";
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
}