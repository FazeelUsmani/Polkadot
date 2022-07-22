// Solve it in two ways
// DON'T let `println!` works
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("Woohoo!");
    
}




// Solve it in two ways
// DON'T let `println!` works
fn main() {
    never_return();

    println!("Failed!");
}

use std::thread;
use std::time;

fn never_return() -> ! {
    
    loop {
        println!("I won't die");
        thread::sleep(time::Duration::from_secs(1))
    }
}
