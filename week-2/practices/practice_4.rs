fn main(){
    let p:f64 = 1000.0;
    let r:f64 = 1.0;
    let t:f64 = 2.0;
c
    // for user clarification of the parameters, i can add something like:
    /* println!("Principal is {}", p);
       println!("Rate is {}", r);
       println!("Time is {}", t);
    */

    // simple interest
    let a = p * (1.0 + (r/100.0) * t);
    println!("Amount is {}", a);
    let si = a - p;
    println!("Simple Interest is {}", si);
}