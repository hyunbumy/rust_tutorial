fn main() {
    // String type dynamically allocates string data in the heap. This is different from a regular
    // string literal type like
    // let s = "hello";
    {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");
    }  // String goes out of scope AND its data is deallocated!
    // When a variable goes out of scope, it calls a "drop" function on the type, which is like the
    // destructor. This, in regards to memory management, is like RAII which C++ smart pointers are
    // an example of.
    
    let s1 = String::from("hello");
    let s2 = s1;  // Makes a copy of the String, which under the hood, contains the pointer to the
                  // data on the heap.
    // After s1 is assigned to s2, s1 is NO LONGER VALID:
    // println!("{s1}"); will throw an error!
    // This is because this assignment is a _move_ like `s2 = std::move(s1);`
    // Rust will never make deep copies by default.

    let mut s = String::from("hello");
    s = String::from("ahoy");  // The initial String is "drop"ed right away as the rvalue goes out
                               // of scope.

    // Use `clone` to deep copy the data
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // How do we know if a type has heap-allocated data or not to treat it with "auto move"? Or does it
    // always treat a new type as having heap-allocated data? ie. is it always "move"ed?
    //   - Stack-only / static data is always copied by value (ie. like primitive types).
    //   - A type (aka class) can be annotated with `Copy` to let the compiler know that it should
    //   be copied. Rust won't let a type be annotated with `Copy` if the type or any of its parts
    //   has implemented the `Drop` trait. `Copy` and `Drop` are how Rust determines what has
    //   heap-allocated data vs not.

    // Ownerships and functions
    let s = String::from("hello");
    takes_ownership(s);
    // s is out of scope as ownership has been transferred into the function ("pass-by-move").
    let x = 5;
    makes_copy(x);
    // x is still in scope even after function since it's passed by copy.

    // Return values are also moved, which is probably actually similar to what happens with C++, etc.
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // For types with potentially heap-allocated data, Rust will always transfer ownership. (ie.
    // each instance is std::unique_ptr, and assigning / parameter will always be used with
    // std::move).
    //   - Note, it probably doesn't need to be thought of as std::unique_ptr since std::move can
    //   still be used for non-pointers.
    //   - Actually, it may be easier to think of all types being moved by default when being moved
    //   around. Only when annotated with `Copy`, will the type be deep copied (or if explicitly
    //   requested via `clone()`).
}

// Heap-allocated like String parameter is passed in by "move" as would be for assignment
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

// Stack-allocated like int parameter is passed in by copy as would be for assignment
fn makes_copy(some_int: i32) {
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(some_str: String) -> String {
    some_str
}

