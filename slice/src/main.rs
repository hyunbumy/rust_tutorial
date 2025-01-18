fn main() {
    // Slice is a referenced _view_ into a contiguous sequence of elementer in a collection.
    let s = String::from("hello world");

    // Slice of String from indices [0, 5)
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let same_slice = &s[..2];  // We can omit the 0 is if starting at index 0.
    
    let len = s.len();
    let slice = &s[3..len];
    let same_slice = &s[3..];  // We can omit the last index
    
    let len = s.len();
    let slice = &s[0..len];
    let same_slice = &s[..];

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // This would be an error since this needs a mutable ref to `s` when `word`, an
    // immutable ref to `s`, is used again, keeping it in the scope.
    // s.clear();  

    println!("The first word is: {word}");

    // A string literal is of type &str, which is a slice pointing to that specific point of the
    // _binary_.  &str is an immutable reference.
    let s = "Hollo world";
    let word = better_first_word(s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    
    let mut a = a;
    a[2] = 10;
    println!("{:?}", a);
}

// &str is a string slice.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Can accept both string slice or ref to String.
fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]

}

fn first_space_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    // The value from the tuple from `enumerate()` returns a ref so we need `&item`
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

