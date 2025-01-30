trait Iterator {
    // Having the associated type forces only one implementation for the given
    // type as opposed to using generics which can allow multiple concrete
    // implementations implementing for different types.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Default Generic type parameter and operator overloading
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// `Add` trait has the following definition
/*
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/
// The default generic type says that without a type specification, it will use
// Self type as the rhs type.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Instead, we can specify a type for the impl.
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Calling same name methods from different traits
/*
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    or

    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    <Dog as Animal>::baby_name();

    If it's an associated function and not a method (ie. no self param).
 */

// Specifying trait dependency
trait OutlinePrint: fmt::Display {
    // OutlinePrint requires the implementation to also impl fmt::Display
}

// Function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Returning closures
// Use a trait object
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // `add_one` is a function pointer of _type_ `fn`
    // This is not a closure which has a trait of `Fn`, which is more like a lambda.
    // Function pointers implement all three of the closure traits.
    let answer = do_twice(add_one, 5);
}
