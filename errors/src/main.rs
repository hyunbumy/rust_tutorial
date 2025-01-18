// Main can also return a Result!
// Will exit with a value of 0 if succes, nonzero otherwise.
fn main() -> Result<(), Box<dyn Error>> {
    // panic will fail the program
    /*
    panic!("crash and burn");

    let v = vec![1,2,3];
    v[99];
    */

    use std::fs::File;
    use std::io::ErrorKind;

    // Returns `Result` enum which is roughly similar to absl::Status
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {error:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {e:?}"),
            },
            // Catch-all that we have named `other_error`.
            other_error => {
                panic!("Unknown problem: {other_error:?}");
            }
        }
    };

    // Using closures
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // value or panic
    // let greeting_file = File::open("hello.txt").unwrap();

    // value or panic with error
    // let greeting_file = File::open("hello.txt").expect("error);

    // Using main Result
    let greeting_file = File::open("hello.txt")?;
    Ok(());
}

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        // We can just return from this context???
        // I guess this is like ASSIGN_OR_ERROR
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// `?` placed after a `Result` does ASSIGN_OR_RETURN
// `?` also goes through the `from` function defined in the `From` trait for errors so it kinda does
// polymorphism stuff for error types.
fn read_username_from_file_macro() -> Result<String, io::Error> {
    // `?` is the real ASSIGN_OR_RETURN macro
    /*
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    */

    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// `?` can also be used for `Option`. Basically for any type that implements `FromResidual`
fn last_char_of_first_line(text: &str) -> Option<char> {
    // Return `None` if `None`, value if `Some`.
    text.lines().next()?.chars().last()
}

// However, ? won't convert between Result and Option.  Use `ok` or `ok_or`.

