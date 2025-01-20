use std::thread;
use std::time::Duration;

fn main() {
    // This will create a new thread that runs the closure.
    let handle: thread::JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{i}");
        thread::sleep(Duration::from_millis(1));
    }

    // Join to guarantee thread completion.
    handle.join().unwrap();

    let v: Vec<i32> = vec![1,2,3];
    // Must have thread take ownership of the vector to guarantee that the value
    // is valid for the duration of the thread execution.
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
    // Rust does not let the thread borrow the value automatically since it does
    // not know how long it will be valid for.
}
