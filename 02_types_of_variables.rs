fn main() {
    // Scalar
    // 1. Integers
    // 2. Floating point numbers
    // 3. Boolean
    // 4. chras
    
    let x: u64 = 5000;
    let x: f32 = 3.14;
    let y: bool = false;
    let y: char = 'x';

    // Memory assigned during compile time

    // Compound
    // 1. tuples are fast to access
    let bootcamp_details: (&str, u64) = ("substrate_sat", 283);
    println!("{}", bootcamp_details.0);
    println!("{}", bootcamp_details.1);
    
    // 2. array
    let arr = [1,2,4,6];
    println!("{}", arr[2]);


}