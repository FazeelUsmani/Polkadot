fn main() {
    let res: bool = is_even(4);
    println!("Res = {}", res);

    let x = if res {100} else { 404 };
    println!("x {}", x);
}

fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        println!("Yes");
        true
    } else {
        println!("No");
        false
    }
}


fn nested(x: i32) {
    if (x < 5) {
        println!("x is less than 5");
    }
}
