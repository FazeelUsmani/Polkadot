fn main() {
    // 1. loop
    // 2. while
    // 3. for in loop
    loop {
    println!("Forever loop");
    break
    }

    let mut c = 0;
    let res = loop {
        c += 1;
        println!("never ending");
        if c == 10 {
            break c * 2;
        }
    };
    println!("never ending = {}", res);

    // can also return values at break statement
    

}