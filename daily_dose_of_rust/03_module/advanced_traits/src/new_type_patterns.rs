use std::fmt;

/*
orphan rule - We can implement a trait on a type as long as
the trait of pattern is defined within our crate

the new type pattern allows us to get around this restriction
by creating a tuple struct with field which is going to be the 
type we are writing this wrapper is local to our crate so we
can implement a new trate for it
*/

pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}