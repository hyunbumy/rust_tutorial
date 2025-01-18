use rand::Rng;
use std::cmp::Ordering;
use std::io; // This is similar to `using` declaration from C++

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // This is a while loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // Pass by mutable ref
            .expect("Failed to read line");

        // It's interesting because the match (aka. switch) statement can return a value or flow
        // control.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("New input requested as error encountered: {error}");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Looks similar to a switch statement
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}
