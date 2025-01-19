fn main() {
    // Box<T> is like std::unique_ptr<T>
    // Box<T> will most often be used for these siutations:
    // * When you have a type whose size can’t be known at compile time and you
    //   want to use a value of that type in a context that requires an exact
    //   size
    // * When you have a large amount of data and you want to transfer ownership
    //   but ensure the data won’t be copied when you do so
    //   - Note that move does a similar thing; refer to
    //     https://stackoverflow.com/questions/17928687/should-i-use-pointers-or-move-semantics-for-passing-big-chunks-of-data
    // * When you want to own a value and you care only that it’s a type that
    //   implements a particular trait rather than being of a specific type
    //   - kind of like polymorphism it seems

    // Allocating data on the heap using Box<T>
    let b: Box<i32> = Box::new(5);
    println!("b = {b}");

    // Enabling Recursive types with Boxes
    // A value of recursive type can have another value of the same type as part
    // of itself.

    // cons list is made up of nested pairs, which could recursively go deep.
    // Rust iteratively goes through each variant to figure out how much space
    // to allocate.  With recursive type, it's deemed "infinitely".
    // To address this, we wrap this in a Box since Box has a static size. This
    // is called "indirection".
    enum List {
      Cons(i32, Box<List>),
      Nil,
    }
    let _list: List = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    // Implementing `Deref` trait allows one to customize the bheavior of the
    // deref operator `*`.  Appropriate implementationg of it allow us to write
    // code that operates on refs and smart pointers.

    let x: i32 = 5;
    let y: &i32 = &x;
    assert_eq!(x, 5);
    // Must deref the ref to make the comparison
    assert_eq!(*y, 5);

    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);
    assert_eq!(x, 5);
    // same with using Box
    assert_eq!(*y, 5);

    // Generic tuple struct
    use std::ops::Deref;
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // Implement Deref trait for deref behavior using `*`
    // Without it, Rust can only deref `&` refs.
    impl<T> Deref for MyBox<T> {
        // Define an "associated type" for the trait to use
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);
    assert_eq!(x, 5);
    // Using `*` here is equivalent to *(y.deref())
    // Extra `*` is necessary since `deref` returns a ref.
    assert_eq!(*y, 5);

    // `Deref coercion` converts a ref to a type that implements the Deref trait
    // into a reference to another type. This happens when we pass a ref to a
    // ref to a particular type's value as an arg to function or method that
    // doesn't match the parameter type.
    fn hello (name: &str) {
        println!("hello, {name}");
    }
    let m: MyBox<String> = MyBox::new(String::from("Rust"));
    hello(&m);
    // Deref trait can be used for mutable refs
    // * From &T to &U when T: Deref<Target=U>
    // * From &mut T to &mut U when T: DerefMut<Target=U>
    // * From &mut T to &U when T: Deref<Target=U>
    // Rust will coerce a mutable ref to immutable, but not the other way
    // around.
}
