
    // This is a custom data type as well.
    enum IpAddrKind {
        V4,
        V6,
    }

fn route(ip_kind: IpAddrKind) {}

enum IpAddr {
    // Attaching data. Does NOT have to be of the same data type.
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    // No data
    Quit,

    // Has named fields like struct
    Move { x: i32, y: i32},

    // The two below is like tuple structs.
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Like structs, we can define associated functions using `impl` for enums.
impl Message {
    fn call(&self) {
        // Define here.
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // Each case is called an `arm`
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Capture the data in enum
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // Enum can be used for mutually exclusive values of a grouping.
    // But different that c++ versions in that they can contain actual data in them as well. For
    // example, it can be used as mutually exclusive group of structs.

    let four = IpAddrKind::V4;
    route(four);
    let six = IpAddrKind::V6;
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Enums can be a way to provide a logical grouping between different structs.

    let m = Message::Write(String::from("hello"));
    m.call();

    // Options
    // Def:
    // // Using generic
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // This is like std::optional<T>
    //
    // Options::Some<i32>
    let some_number = Some(5);
    // Options::Some<char>
    let some_char = Some('e');
    
    let absent_number: Option<i32> = None;

    // `match` control flow
    // Similar to `switch` in c++ but more expressive.
    println!("Value for coin {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Matches must be exhaustive.
    // Use `other` (if we use the value) or `_` (if value is not needed) for catch-all
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    let dice_roll = 8;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    let dice_roll = 6;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Nothing happens
        _ => (),
    }

    // Use `if let` to handle values that match ONE pattern while _ignoring_ the rest.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max is configured to be {max}"),
        _ => (),
    }
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        // In order to mark the data in pattern matchin is borrowed (and not moved), we must annotate
        // it with `ref`
        Coin::Quarter(ref state) => println!("State qurter from {state:?}!"),
        // Notice that we don't need any capturing or something for these.
        _ => count += 1,
    }
    let mut count = 0;
    if let Coin::Quarter(ref state) = coin {
        println!("State qurter from {state:?}!");
    } else {
        count += 1;
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

