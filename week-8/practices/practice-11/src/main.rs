fn main() {
    let numners = [1, 2, 3, 4, 5];
    println!("original array: {:?}", numners);

    let slice1 = &numners[1..3];
    println!("2nd and 3rd elements sliced: {:?}", slice1);

    let slice2 = &numners[..3];
    println!("0 to index 3 sliced: {:?}", slice2);

    let slice3 = &numners[2..];
    println!("2nd index to 5 end sliced: {:?}", slice3);

    let slice4 = &numners[..];
    println!("entire array sliced: {:?}", slice4);
}
