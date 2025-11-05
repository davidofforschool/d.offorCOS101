fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("value of a: {}", a);
    println!("Value of b: {}", b);

    let mut res = a>b;
    println!("a greater than b", res);

    res = a<b;
    println!("a less than b", res);

    res = a>=b;
    println!("a greater than or equal to b", res);

    res = a<=b;
    println!("a less than or equal to b", res);

    res = a==b;
    println!("a equals b", res);

    res = a!=b;
    println!("a doesnt equal b", res);
}
