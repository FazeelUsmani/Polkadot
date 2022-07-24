
// Fix error with at least two solutions
// You ask 2, wait I'll give you 3

fn main() {
    let s = "hello, world";
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}",s)
}



// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s.to_owned())
}

fn greetings(s: String) {
    println!("{}",s)
}



// Fix error with at least two solutions
fn main() {
    let s = String::from("hello, world");
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}


