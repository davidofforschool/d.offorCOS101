fn main() {
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of number is: {}", num);
}

fn mutate_num_to_zero(mut param_num: i32) {
    param_num = param_num * 0;
    println!("The value of param_num is: {}", param_num);
}