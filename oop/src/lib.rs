// Define a trait that other types can implement.
// This functions similar to polymorphism and limited inheritance.
// ie. Java's interface
// We create a trait object by specifying some sort of pointer such as a & ref
// or a Box<T>, then the `dyn` keyword for "dynamic" type.
// 
// In Rust, enums and structs are not really "objects" since their data and methods
// are separated.
// Trait objects are more similar to traditional objects that have both data and
// methods encapsulated.
pub trait Draw {
    // Define the API of the trait.
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("blah");
    }
}

pub struct Screen {
    // Use the trait object to stand in for generic or concrete type.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        // This is different than generic type param with trait bounds since
        // those can only be substituted with one concrete type at a time.
        // Trait objects allow for multiple concrete types to fill in at runtime.
        // Hence, if you need a homogenous collections, etc, generics and trait
        // bounds is preferable.
        // Otherwise, may be preferable to use trait.
        for component in self.components.iter() {
            component.draw();
        }
    }
}
