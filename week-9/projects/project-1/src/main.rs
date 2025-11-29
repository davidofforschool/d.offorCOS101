use std::fs::File;
use std::io::{self, Write};

fn main() {
    // Vectors for each drink category
    let mut lager: Vec<String> = Vec::new();
    let mut stout: Vec<String> = Vec::new();
    let mut non_alcoholic: Vec<String> = Vec::new();

    println!("Enter Lager drinks (type 'done' to finish):");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let drink = input.trim().to_string();
        if drink.eq_ignore_ascii_case("done") { break; }
        lager.push(drink);
    }

    println!("\nEnter Stout drinks (type 'done' to finish):");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let drink = input.trim().to_string();
        if drink.eq_ignore_ascii_case("done") { break; }
        stout.push(drink);
    }

    println!("\nEnter Non-Alcoholic drinks (type 'done' to finish):");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let drink = input.trim().to_string();
        if drink.eq_ignore_ascii_case("done") { break; }
        non_alcoholic.push(drink);
    }

    // Save to file
    let mut file = File::create("drinks.txt").expect("Could not create file");

    writeln!(file, "LAGER DRINKS:").unwrap();
    for d in &lager {
        writeln!(file, "- {}", d).unwrap();
    }

    writeln!(file, "\nSTOUT DRINKS:").unwrap();
    for d in &stout {
        writeln!(file, "- {}", d).unwrap();
    }

    writeln!(file, "\nNON-ALCOHOLIC DRINKS:").unwrap();
    for d in &non_alcoholic {
        writeln!(file, "- {}", d).unwrap();
    }

    println!("\n File 'drinks.txt' created successfully!");
}