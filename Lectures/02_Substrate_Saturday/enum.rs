/*
This is precluded
enum Option <T> {
    Some(T), 
    None
}
*/

pub cons SESSION_NAME: &str = "Traits";   // Global scope - no erros when unused

fn main() {
    let x: Option<i32> = Some(10);
    println!("{:?}", x);
    let y = 34;     // unused variable
}