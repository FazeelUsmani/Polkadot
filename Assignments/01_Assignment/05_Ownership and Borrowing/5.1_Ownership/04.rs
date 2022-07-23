// Fix the error without removing code line
fn main() {
    let mut s = String::from("hello, world");

    s = print_str(s);

    println!("{}", s);
}

fn print_str(s: String) -> String {
    println!("{}",s);
    s
}
