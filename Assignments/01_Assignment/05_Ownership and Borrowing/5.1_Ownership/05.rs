// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


// Use clone() when you're copying String else copy is enough