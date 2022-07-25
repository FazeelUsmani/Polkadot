mod iterator_trait;
mod generic_type_parameters;
mod methods_with_same_name;
mod super_traits;
mod new_type_patterns;

use iterator_trait::*;
use generic_type_parameters::*;
use methods_with_same_name::*;
use super_traits::*;
use new_type_patterns::*;

fn main() {
    println!("Advanced traits in rust");

    /*
    let mut x = Counter {};
    let y: u16 = x.next().unwrap();

    println!("{:?}", y);
    */

    /*
    assert_eq!(
        Point { x: 1, y: 0} + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    println!("Success!");
    */
    /*
    assert_eq!(
        Millimeters(1000) + Meters(1),
        Millimeters(2000)
    );
    println!("Success!");
    */
    /*
    let person = Human;
    person.fly();
    // Here giving reference to trait then function
    Pilot::fly(&person);
    Wizard::fly(&person);
    */

    // Associated functions can be called without reference
    // Human::fly();
    // <Human as Pilot>::fly();
    // <Human as Wizard>::fly();

    // SecondPoint {x: 4, y: 5}.outline_print();

    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w = {}", w);

}
