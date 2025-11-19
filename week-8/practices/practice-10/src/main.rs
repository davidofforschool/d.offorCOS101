fn main() {
    let b:(i32, bool, f64) = (42, true, 4.89);
    print(b);
}

fn print(x:(i32, bool, f64)) {
    let (age, is_male, cgpa) = x;
    println!("age is: {}, isMale? {}, cgpa is: {}", age, is_male, cgpa);
}