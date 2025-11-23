use std::io;

fn main() {
    
    let mut candidates: Vec<(&str, u32)> = vec![
        ("David", 4),
        ("Naomi", 7),
        ("Emeka", 3),
        ("Fatima", 10),
        ("Chidi", 6),
    ];

    
    candidates.sort_by(|a, b| b.1.cmp(&a.1));

    
    let (name, years) = candidates[0];

    println!(
        "The developer with the highest programming experience is {} with {} years.",
        name, years
    );
}