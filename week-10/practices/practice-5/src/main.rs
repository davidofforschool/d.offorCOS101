fn borrow_vector(z:&Vec<i32>) {
    println!("**************************");
    println!("Inside borrow_vector function {:?} \n", z);
    println!("--------------------------");
}

fn main() {
    let v = vec![100, 200, 300];
    borrow_vector(&v);

    println!("Printing the value from main() x[0]={}", v[0]);
    println!("**************************");
}