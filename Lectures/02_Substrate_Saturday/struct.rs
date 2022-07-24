pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        // Person(name: name, age: age)     // normal way
        Person(name, age)      // shorthand
    }

    // method
    pub fn greet(&self) -> String {
        format!("Hi my name is {}", self.name);
    }

    pub fn add(i: i32, y: i32 ) -> i32 {
        x + y
    }

    pub fn age_up(&mut self, x: i32) {
        self.age += n;
    }
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}


/*

self -> can mutate read + write
&self -> Only reading
&mut self -> mutable reference
mut self ->  

*/

fn main() {

    let x = Person::new("Alice".to_string(), 5);
    x::add(3,4); // 7
    x.age_up(10);

    println!("{}", x.age);

    let z = get_age(&x);
    println!("{}", z);

    x::greet("Hello brother".to_string());

}