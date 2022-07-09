fn main() {
    println!("Hello World!");

    // By default i32 datatype
    let x = 5;
    println!("{}", x);
    // Shadowing - redeclaring a variable using let
    let x: i8 = 127;
    println!("{}", x);

    // x = 6;  // This won't work - variable is immutable
    // You can make variable mutable using mut keyword
    // println!("{}", x);
    
    let mut x = 10;
    println!("{}", x);
    
    x = 12;
    println!("{}", x);
    
    const PARTICIPANT_COUNT: i32 = 283;
    println!("{}", PARTICIPANT_COUNT);
    
    
    let y = 5;
    println!("{}", y);
    // shadowing
    let y = "Fazeel Usmani";
    println!("{}", y);


    



}