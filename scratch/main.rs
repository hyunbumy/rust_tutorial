fn main() {
    let mut some_str = String::from("hello");
    change_ref(&mut some_str);
}

// We can still specify whether the parameter itself should be mutable or immuable (ie. `const`) by
// specifying `mut` in front of the parameter name, while immutability of the Type is speficied
// along with the type declaration.
fn change_ref(mut s: &mut String) {
    let mut new_str = String::from("new_string");
    s = &mut new_str;
    println!("{s}");
}

