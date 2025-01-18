struct User {
    active: bool,
    // We use String here instad of &str for string slice since we want the String to be owned by
    // the struct instance.
    // A field CAN be a ref, but needs to specify a _lifetime_.
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // Use "field init shorthand" to not have to specify the same name variable to the field.
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple struct: essentially a named tuple type
struct Color(i32, i32, i32);

// Unit-Like struct without any associated data.
struct AlwaysEqual;

#[derive(Debug)]  // Adding a `Debug` attribute, allowing println!
struct Rectangle {
    width: u32,
    height: u32,
}

// impl defines implementation for the Rectangle struct
// Any functions within the impl are called associated functions, which methods are a type of.
impl Rectangle {
    // methods - Has self as the first parameter kinda similar to Python
    // `&self` is short for `self:&Self`, and Self is an alias for the type that the `impl` block
    // is for.
    // Same ownership rules apply for the self parameter though taking ownership of self is rarer.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }

    // Associated function (aka no `self`). Kinda like static methods
    // We can still use `Self` type since it's within the impl.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // We can have a method of the same name as the field
    // Usually these would be getters though.
    fn width(&self) -> bool {
        self.width > 0
    }
}

// There can be multiple `impl` blocks for the same type.
impl Rectangle {
    fn nothing() {}
}

fn main() {
    // The entire instance must be mutable or immutable.
    let user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someusername1234"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);

    // Use the struct update syntax to create a new, modified instance
    // `..` specifies that the remaining fields not explicitly set should have the same value as
    // the fields in the given instance.
    // Using a struct update syntax uses a move though for struct fields that does not have
    // `Copy` trait.  Therefore, we cannot use `user1.username` cannot be used since it has been
    // moved, while fields that were other copied or newly created in user2 can be!
    let mut user2 = User {
        email: String::from("another@example.com"),
               ..user1
    };
    user2.username = String::from("anotheruser");
    // This succeeds since user1.username was _not_ moved.
    println!("{}", user2.username);
    // `println!("{}", user1.username)` would fail

    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
    println!("{}", user3.email);

    let black = Color(0, 0, 0);
    println!("{}", black.2);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };
    // Print using a debug format for the struct
    println!("rect1 is {rect1:?}");
    // Pretty-print
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The rectangle has a nonzero width; it is {}", rect1.width);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("{square:?}");
}
