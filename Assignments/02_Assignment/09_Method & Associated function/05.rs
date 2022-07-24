
struct Rectangle {
    width: u32,
    height: u32,
}

// Using multiple `impl` blocks to rewrite the code below.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn isSquare(&self) -> bool {
        self.width == self.height
    }
}


fn main() {
    
    let rect = Rectangle {
        width : 30,
        height : 40
    };
    
    println!("{}", rect.area());
    
    
    let rect2 = Rectangle{
        width: 10,
        height : 10
    };
    
    println!("Is rect2 a square? Ans is {}", rect2.isSquare());
    
    
    println!("Can rect1 hold rect2? Ans is {}", rect.can_hold(&rect2));
    
    
    println!("Success!");
}
