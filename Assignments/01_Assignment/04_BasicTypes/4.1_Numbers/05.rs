
// Fix errors and panics to make it work
fn main() {
    let v1 = 21_u8 + 8;
    let v2 = i8::checked_add(25, 8).unwrap();
    println!("{},{}",v1,v2);
 }
 