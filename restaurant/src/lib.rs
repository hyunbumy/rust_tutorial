// This is a crate root. src/main.rs or src/lib.rs create a top-level module named `crate`

// we can define modules here
// Modules, by default, are private to parent scope from where they are defined.

/*
mod front_of_house {
    // Within it, we can define submodules
    pub mod hosting {
        // Same visibility rule with functions
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
*/

mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        // public field
        pub toast: String,
        // private field
        seasonal_fruit: String,
    }

    // In contrast, public enum has all its values as public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        // We need a constructor since one of the fields in the struct is a private field and
        // cannot be accessed.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


// Path can take two forms:
//   - absolute path startimg from the create root
//   - relative path starting from current module, using `self`, `super`, or identifier in the current //     module.

// Bring in the module into the scope using `use`.  Seems like a combo of `import` from Python and
// `using` from c++.
// Only creates the shortcut for the particular scope in which the `use` occurs.
//
// When specifying shortcuts via `use`, bring the whole module if for functions, or specify the
// specific structs, enums, or others using the full path to the item being brought in.
use crate::front_of_house::hosting;
// The exception is if we are bringing two items with the same name into scope. In that case, use
// the module.
use std::fmt;
use std::io;
// fmt::Result or io::Result

//  Or specify a new name with the `as` keyword.
use std::fmt::Result;
use std::io::Result as IoResult;

// we can also make the names public so that external dependency can more easily call into the
// modules
pub use crate::front_of_house::hosting; // we can use restaurant::hosting::add_to_waitlist()
                                        // instead of restaurant::front_of_house::hosting::add_to

// Bring in external packages using `use`
use std::collections::HashMap;
// You may also need to specify the dependency in Cargo.toml
use rand::Rng;

// If we are importing a lot of stuff from the same module, we can shorthand:
use std::{cmp::Ordering, io};
use std::io::{self, Write};  // from `use std::io; use std::io::Write;`

// We can also bring in all public items from a path using wildcard
// use be used judiciously, with the most common being for tests module.
use std::collections::*;

// We can access front_of_house module here even without it being pub since they are "siblings"
// (defined in the same module).
pub fn eat_at_restaurant() {
    // absolute path; generally prefer absolute.
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // With `use`
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Access public field
    meal.toast = String::from("Wheat");
    // Cannot access private field
    // meal.seasonal_fruit = String::from("blueberries");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Essentially, mark pub for the public API (functions, modules, structs, etc.).

