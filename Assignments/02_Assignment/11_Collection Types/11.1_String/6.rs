fn main() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for i in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}