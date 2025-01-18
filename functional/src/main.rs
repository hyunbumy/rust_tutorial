#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // `|| self.most_stocked()` is a closure
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closure definition
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Closures can CAPTURE values by (1) borrowing immutably (2) borrowing mutably (3) taking
    // ownership.
    
    // Capture with immutable borrow
    let list = vec![1,2,3];
    let only_borrows = || println!("From closure: {list?}");
    only_borrows();

    // capture with mutable borrow
    let mut list = vec![1,2,3];
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();

    // capture with ownership
    let list = vec![1,2,3];
    // A separate thread needs ownership of the captured data since it's undeterministic which
    // thread would finish first.
    thread::spawn(move || println!("from thread: {list:?}")).join().unwrap();

    // A closure body can do any of the following:
    // - Move a captured value out of the closure (ie. moving references / ownership out)
    // - Mutate the captured value
    // - Neither move nor mutate
    // - Capture nothing from the beginning
    // The way a closure captures and handles values from the env affects which traits the closure
    // implements, and traits are how func and structs specify what kind of closures they can use.
    // 
    // FnOnce applies to closures that can be called once
    // - All closures define at least this.
    // - Moving captured values out of its body will only implement this
    //
    // FnMut applies to closures that don't move captured values out, but _might_ mutate. Can be
    // called multiple times
    //
    // Fn applies to closures that neither move nor mutate / no capturing. Can be called multiple
    // times

    // Iterators perform some tasks on a sequent of items in turn.
    // Iterators are lazy; no effect until methods are called that consume te iterator.
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got {val}");
    }

    // All iterators implement a trait `Iterator` and define `next` method.
    // `next` returns `Some<Type>` if value, `None` otherwise.
    // `next` is mutating
    // Can call `iter()` for immutable ref iteration, `into_iter` for owned values, or `iter_mut`
    // for mutable ref.

    // Methods that call `next` are called `consuming adapters`. Iterators cannot be used after being
    // `consumed`.
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    // Iterators can be called with `iterator adapters` that will not consume the iterator and
    // instead produce different iterators.
    let v1: Vec<i32> = vec![1,2,3];
    // Iterators must be used / consumed.
    let v2:Vec<_> = v1.iter().map(|x| x + 1).collect();

    // Many iterator adaptors take closures as arguments and they usually capture their env.
}

