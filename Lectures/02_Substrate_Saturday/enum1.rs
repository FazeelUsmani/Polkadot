/*
This is preluded
enum Result<T, E> {
    Ok(T),
    Err(E)
*/
}

fn main() {
    let x = Result <i32, i32> = Ok(1);
    println("{}", x);  // Ok(1)
    
    let y = Result <i32, i32> = Err(3);
    println("{}", y);  // Err(3)

}