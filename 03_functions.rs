fn main() {

    let res = func1(32, 64);
    println!("Res = {}", res);
}

fn func1(x: i32, y: i32) -> i32 {   // -> i32 says return
    // Line 8 and 9 are statements
    println!("In func1");
    println!("x = {}, y = {}", x, y);

    // Expression
    let mul = x * y;
    // println!("Multiplication is {}", mul);

    // return mul;
    mul   

    /*
    There are 2 types of returning a variable
    1. return mul;
    2. mul;
    */

}