fn main() {
    let mut colors = ["Red", "Green", "Yellow", "Purple"];
    println!("Original colors: {:?}", colors);

    let sliced_colors = &mut colors[1..3];
    println!("First slice: {:?}", sliced_colors);

    sliced_colors[1] = "White";
    println!("Changed slice = {:?}", sliced_colors);
}
