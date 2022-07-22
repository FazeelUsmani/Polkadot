
// Fix the error with the use of define_x
fn main() {
    println!("{}, world", define_x()); 
}

fn define_x() -> String {
    let x = "hello";
    x.to_owned()
}
