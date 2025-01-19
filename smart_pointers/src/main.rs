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
    let list: List = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
}
