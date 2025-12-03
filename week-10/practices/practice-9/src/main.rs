struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let small = Rectangle { width: 10, height: 5 };
    
    println!("The width is {}, The height is {}, The area is {}", small.width,small.height,small.area());
}