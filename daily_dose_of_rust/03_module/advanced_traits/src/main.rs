mod iterator_trait;

use iterator_trait::*;

fn main() {
    println!("Advanced traits in rust");

    let mut x = Counter {};
    let y: u16 = x.next().unwrap();

    println!("{:?}", y);


}
