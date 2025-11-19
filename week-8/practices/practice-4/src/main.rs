fn main() {
    let name = vec!["Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Heidi"];

    let age = vec![23, 30, 25, 28, 22, 27, 24, 29];

    for i in 0..age.len() {
        println!("{} is {} years old.", name[i], age[i]);
    }
}
