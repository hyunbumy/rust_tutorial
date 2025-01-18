fn main() {
    let number = 7;


    // if statement must be using an expression that evalutes strictly to bool
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    } else {
        println!("something else here");
    }

    let condition = true;
    // Interesting way to assign using if statements; note the parentheses
    // These are essentially if _expressions_ that return a value.
    let number = if condition { 
        println!("Ahoy!");
        5
    } else { 6 };
    println!("The value of the number is: {number}");

    // This does not compile since the if-else must evaluate to a value of the same type.
    // let number = if condition { 5 } else { "six" }
    
    // Like `while(true)`
    let mut count = 1;
    loop {
        if count > 5 {
            break;
        }
        println!("again!");
        count += 1;
    }

    // This functions as a generator type in Python by having loop able to return values
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            // Like `return` but for loops
            break count * 2;
        }
    };
    println!("the result is {result}");

    let mut count = 0;
    // Named loops using loop labels
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // Will break the innermost loop
                break;
            }
            if count == 2 {
                // Woah, we can actually break out of the named outerloop from innerloop??
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count = {count}");

    // Proper while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping through collection
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        // Need to extract this out, and not have it as an inline variable.
        println!("the vlue is: {}", a[index]);
        index += 1;
    }

    // range based for-loop similar to python
    let a = [10, 20, 30, 40 ,50];
    for element in a {
        println!("the value is {element}");
    }

    // (x..y) is a Range generating a sequence of numbers [x, y]
    // .rev() reverses the order
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}

