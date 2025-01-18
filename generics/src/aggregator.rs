// Trait defines the functionality a particular type has and can share with other types. This is
// similar to "interfaces" in other languages.
//
// Trait definitions are a way to group method signatures together to define a set of common
// behaviors.

pub trait Summary {
    fn summarize(&self) -> String;

    // Can define a default implementation
    // We CANNOT call the default implementation from an overriding implementation of the same
    // method.
    fn default_fn(&self) -> String {
        format!("default {}", self.summarize())
    }
}

// Implementing a Trait on a Type
pub struct NewsArticle {}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} hello", "wow")
    }
}

// In order to implement a trait, at least one of the type or the trait must be local to the crate.
// For example, if the trait AND the type are defined in a different crate, we cannot combine the two
// here. This is called coherence -> orphan rule. This is so that other ppls code cant break yours,
// vice versa.

// Trait can be used in the parameter similar to inheritance.
// This is actually a shorthand for `pub fn notify<T: Summary>(item: &T) {}` -> trait bound
pub fn notify(item: &impl Summary) {
    // Parameter of item of type that implements Summary

    println!("Breaking news! {}", item.summarize());
}

// This would be more appropriate if we want to keep the types of the two params the same.
// Otherwise, we could simply use `impl Trait`
pub fn notify_two<T: Summary>(item1: &T, item2: &T) {}

// Specify more traits
pub fn notify_more(item: &(impl Summary + Display)) {}
pub fn notify_more_generic<T: Summary + Display>(item: &T) {}

use std::fmt::Debug;

// Instead of `fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}`
pub fn some_function<T, U>(t: &T, u: &U) -> i32 
where
    T: Display + Clone,
    U: Clone + Debug,
{3}

// Return type trait bound
// `impl Trait` on return type doesn't allow compile-time difference in the return objects
// ie. we can't conditionally return NewsArticle or Tweet at compile time _using `impl Trait`_.
fn returns_summarizable() -> impl Summary {
    NewsArticle{}
}

// We can use trait bounds to conditionally implement generics methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also use it to conditionally implement a trait for any type that implements another trait
// impl<T: Display> ToString for T {}

