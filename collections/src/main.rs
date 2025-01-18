fn main() {
    let v: Vec<i32> = Vec::new();
    // vec! is a macro
    // Type inference works due to the default values.
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("the third element is {third}");
    // Or we can return the actual moved element.
    // This would only work since it's a vector of a primitive type which implements `Copy` trait
    let third = v[2];

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("there is no third element"),
    }

    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(6);
    // println!("{first}");  This will fail since pushing an element in Vec will get a mutable ref
    // to the first element as well as the entire content may be moved. Hence it cannot coexist
    // with the immutable ref from earlier.

    let v = vec![100, 32, 57];
    // We could do a move of v instead (ie. without &), but would would not be able to reference it
    // again.
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 50];
    for i in &mut v {
        // Accessing / modifying the elements is safe since it follows the borrow checker's rules.
        *i += 50;
        // adding to the vector itself is not
        // v.push(20);
    }

    // Use an enum to store a vector of different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1,2,3,4];
    }  // v goes out of scope and is freed
       // When it does, all its contents get dropped as well.

    // Rust has only one string type in the core language which is the string _slice_ `str`.
    // It is often seen as &str.
    
    let mut string = String::new();
    let data = "ininital comments";
    // `to_string` is available for any type that implements `Display` trait.
    let s = data.to_string();
    let s = "initial contents".to_string();
    // Strings are UTF-8 encoded so each character is 4bytes instead of the usual 1 byte from c++.
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    // Takes string slice
    s.push_str("bar");
    
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 gets moved while s2 is ref'ed. This is actually due to the signature
                        // of `add` method which `+` uses.
    // Compiler can coerce the &String argument to &str called `deref coercion`

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    // Rust does not support indexing a String due to UTF-8 encoding
    // However, we can specify the byte range of String to create a str slice.
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // If it's a bad range (ie. only a part of character's bytes), it will panic.

    // Iterating over String
    // Iterate specifically with chars
    for c in "3안".chars() {
        println!("{c}");
    }
    // Iteracting over bytes
    for b in "3안".bytes() {
        println!("{b}");
    }

    use std::collections::HashMap;

    // Type inference works due to the subsequent usage of the hashmap.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    // get returns Option<&V>
    // copied() returns Option<V>
    // unwrap_or is essentially value_or for std::optional<T>
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement the `Copy` trait, the keys values are copied into the hash map. For
    // owned values, the keys and values will be moved into the hashmap.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid now.

    // We can also insert references to values for keys or values of hashmap which won't move the
    // underlying data.  They must be valid for at least as long as the hashmap is valid.
    
    // Overwrite value.
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Blue", 25);
    println!("{scores:?}");

    // Insert if not found using `entry()`; returns an enum called Entry that represents a value
    // that may or may not exist in the HashMap.
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.entry("Yellow").or_insert(50);
    scores.entry("Blue").or_insert(50);
    println!("{scores:?}");

    // Updating value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns a mut ref to the value (&mut V)
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
