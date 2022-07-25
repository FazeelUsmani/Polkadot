pub trait Pilot {
    // associated functions
    fn fly();
    // fn fly(&self);
}

pub trait Wizard {
    // associated functions
    fn fly() ;
    // fn fly(&self);
}

pub struct Human;

impl Human {
    // associated functions
    fn fly() {
        println!("Human fly - associated function")
    }
    // pub fn fly(&self) {
    //     println!("*Waving arms furiously*");
    // }
}

impl Pilot for Human {
    // associated functions - should not be public because they're implied
    fn fly() {
        println!("Pilot fly - associated function")
    }
    // fn fly(&self) {
    //     println!("This is your captain speaking");
    // }
}

impl Wizard for Human {
    // associated functions
    fn fly() {
        println!("Wizard fly - associated function")
    }
    // fn fly(&self) {
    //     println!("Up!");
    // }
}