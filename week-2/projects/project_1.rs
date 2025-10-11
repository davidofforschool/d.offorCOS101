fn main(){
    let p:f64 = 520_000_000.00;
    let r:f64 = 10.00;
    let n:f64 = 5.00;

    //compound interest calculation

    let a = p * (1.0 + (r/100.0)).powf(n);
    println!("Amount is {}", a);
    let ci = a - p;
    println!("Compound Interest is {}", ci);
}