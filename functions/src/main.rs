fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let plus_one = plus_one(7);
    println!("Value is {plus_one}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; does not compile since having a semi-colon turns it into a _statement_ rather than an
    // _expression_.
}

