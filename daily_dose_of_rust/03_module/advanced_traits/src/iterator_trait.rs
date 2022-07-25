/*pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}



impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}
*/
// Implementing generics to implement the generic type
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

pub struct Counter {}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}
impl Iterator<u128> for Counter {
    fn next(&mut self) -> Option<u128> {
        Some(0)
    }
}

