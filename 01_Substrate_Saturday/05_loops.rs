fn main() {
    // 1. loop
    
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
    
    // 2. while
    while c != 0 {
        println!("something");
        c -= 1;        
    } 

    println!("something ending = {}", c);

    // 3. for in loop
    let arr = [11,22,33,44,55];
    for i in arr.iter() {
        println!("ele is {}", i);
    }
    println!(">>>>>>><<<<<<<<");
    for i in arr {
        println!("ele is {}", i);
    }
    
    println!(">>>>>>><<<<<<<<");
    for i in (1..4).rev() {
        println!("ele is {}", i);
    }
    
    println!(">>>>>>><<<<<<<<");
    for i in (1..=4).rev() {
        println!("ele is {}", i);
    }
    

}