//! This kind of comment describes the item that _contains_ the comments rather than to the items
//! following the comments.
//!
//! Conventionally inside the crate roote file.
fn main() {
    println!("Hello, world!");
}

/// This is a documentation comment. Use `cargo doc --open` to build the HTML for the
/// documentation.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// ```
///
/// # Panics
///
/// Scenarios in which the function could panic
///
/// # Errors
///
/// Describing the kinds of errors that might occur and what conditions
///
/// # Safety
///
/// If function is `unsafe`
pub fn add_one(x: i32) -> i32 {
    x + 1
}
