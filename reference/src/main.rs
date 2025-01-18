fn main() {
    let s1 = String::from("hello");
    let len = calc_length(&s1); // Pass the parameter as a ref

    // s1 is still in scope as it has not transferred ownership.
    println!("The length of '{s1}' is {len}");

    let mut s = String::from("hello");
    change(&mut s); // This is creating a _new_ mutable ref based on _mutable_ s.
                    // Cannot create a mutable ref from an immutable instance.

    // It seems like [im]mutable refs are treated as their own "types" and are not automatically
    // compatible with their non-ref counterparts.

    let mut s = String::from("hello");

    {
        // There can only be ONE mutable ref referencing an instace at the same time.
        let r1 = &mut s; // It's an immutable instance of a mutable ref (think of it as `Type
                         // *const ptr`);
        r1.push_str(", world1");
        println!("{}", r1);
    }
    // This is okay since r1 is out of scope (while it wouldn't have been if two were both in scope.
    let r2 = &mut s;
    r2.push_str(", world2");
    println!("{}", r2);

    let mut s = String::from("hello");
    // There can be as many immutable refs to the same instance as we want
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    // However, we cannot have an immutable ref and mutable ref within the scope.
    // let r3 = &mut s;
    // println!("{}, {}, {}", r1, r2, r3);

    // However, note that a references' scope starts from were it's introduced through to the
    // _last time_ they are _used_.
    // So something like this is legal
    let r1 = &s;
    println!("{r1}");
    // r1 is out of scope now since it's not used anymore.

    let r2 = &mut s;
    println!("{r2}");  // This is legal since r1 is no longer used and hence out of scope.
    // These are probably like this with multithreading in mind.  So think of them as such.

    //  - At any given time, you can have either one mutable reference or any number of immutable
    //  references.
    //  - References must always be valid.
    //
    //  As a rule of thumb, think of references as a std::unique_ptr.
    //  let mut s = String::from("hello");
    //  let ref = &s; -> const std::unique_ptr<const String>
    //  let ref = &mut s; -> const std::unique_ptr<String>
    //  let mut ref = &s; -> std::unique_ptr<const String>
    //  let mut ref = &mut s; -> std::unique_ptr<String>
    //  When calling methods or variables on refs, we don't need an `->` equivalent since Rust does
    //  this automatically.

}

// Receive parameter as a reference and thus does not assume ownership.
// This is called borrowing.
// Equivalent to C++ `void func(const Type& type)`.
fn calc_length(s: &String) -> usize {
    // Note that we cannot modify s unless marked as mutable.
    s.len()
}

// Mutable reference. Equivalent to C++ `void function(Type& type)`.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
