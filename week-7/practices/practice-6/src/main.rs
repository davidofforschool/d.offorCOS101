fn main() {
    let arr1[i32;4] = [10, 20, 30, 40];
    println!("array with data type");
    println!("array is {:?}", arr1);
    prinln!("array size is : {}", arr1.len());

    // array without data type (implicit float datatype)
    let arr2 = [10.5, 20.5, 30.5, 40.5];
    println!("array without data type");
    println!("array is {:?}", arr2);
    println!("array size is : {}", arr2.len());

    // array with default values that creates and initializes all its elements with a default value of -1
    let arr3[i32;8] = [-1; 8];
    println!("array with default values");
    println!("array is {:?}", arr3);
    println!("array size is : {}", arr3.len());
}