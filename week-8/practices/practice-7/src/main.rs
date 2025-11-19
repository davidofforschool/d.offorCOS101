fn main() {
    let datatype_tuple: (&str, f32, u8) = ("Rust", 1.56, 8);
    println!("Tuple contents: {:?}", datatype_tuple);

    let no_datatype_tuple = ("Rust", 1.56, 8);
    println!("Tuple contents without explicit types: {:?}", no_datatype_tuple);

    println!("First element: {}", datatype_tuple.0);
    println!("Second element: {}", datatype_tuple.1);
    println!("Third element: {}", datatype_tuple.2);
}
