
// Modify this struct to make the code work
struct Point<T, W> {
    x: T,
    y: W,
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}
