fn main() {
    let city_arr:[&str;5] = ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix"];
    println!("array is {:?}", city_arr);
    println!("array length is {}", city_arr.len());

    for index in 0..5 {
        println!("city at index {} is {}", index, city_arr[index]);
    }
}