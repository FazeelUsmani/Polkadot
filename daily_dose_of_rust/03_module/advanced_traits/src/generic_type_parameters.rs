use std::ops::Add;

/*
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
 */

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Millimeters(pub u32);

#[derive(Debug, PartialEq)]
pub struct Meters(pub u32);    // puble is immutable

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


