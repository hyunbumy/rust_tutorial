// Generics in functions
// Can use any identifier as a type parameter name
// we can restrict what T can be
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generics in structs
struct Point<T, U> {
    x: T,
    y: U,
}

// Must declare generics after `impl` to tell compiler that we are implenting for generics Point
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Methods for specific generic types.
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// <X1, Y1> relevant to the struct definition
impl<X1, Y1> Point<X1, Y1> {
    // <X2, Y2> relevant to the method definition.
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Enums can have generics as well.

mod aggregator;

// we need to bring in both the trait and the type.
use crate::aggregator::{Summary, NewsArticle};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let both_integer = Point {x:5, y: 10};
    let both_float = Point {x:1.5, y: 1.0};
    let int_and_float = Point {x:1, y: 1.0};

    let news = NewsArticle{};
    println!("News article: {}", news.summarize());

    // Every ref has a `lifetime` which is the scope for which that ref is valid for.  Annotating
    // lifetime is to prevent dangling refs
    //
    // Lifetime annotation has a similar syntax as generics.

    let r: i32;  // This is valid as long as we give this a value before it's actually used.
    // This will fail due to r referencing out of scope / invalid data.
    /*
    {
        let x = 5;
        r = &x;
    }
    println!("r: {r}");
    */

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
    */
    // The print statement will fail since the lifetime of the result value is the lifetime of string2
    // which is the lesser of the two inputs, and result is used outside of that lifetime.

    // Static lifetime lives for the entire duration of time.
    // All string literals have static lifetime implicitly since they live in the binary.
    let s: &'static str = "I have static lifetime";

    // Altogether
    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() {
            return x;
        }
        y
    }
}

// Lifetime annotation does not change how long any of the references live.  Instead, they describe
// the relationships of lifetime sof multiple references to each other. These annotations then help
// the compiler determine validity of the references at compile time.
//
// We need to annotate the lifetime for the return value since it doesn't know whether it should
// take the lifetime of str1 or str2 which may differ.
// By using the same lifetime annotation `'a` for both inputs and output, we tell the compiler that the
// return ref is valid as long as both the params are valid.
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    }
    str2
}

// Structs can hold references, but they would need to be lifetime-annotated for every ref in the
// struct def.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime annotations in method def
impl<'a> ImportantExcerpt<'a> {
    // No lifetime annotation required here since one param gets an implicit lifetime (1st elision
    // rule)
    fn level(&self) -> i32 {
        3
    }

    // No lifetime annotation required since two params get their own lifetime and return gets the
    // lifetime of self (3rd elision rule)
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("something");
        self.part
    }
}

