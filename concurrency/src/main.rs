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

    let v: Vec<i32> = vec![1, 2, 3];
    // Must have thread take ownership of the vector to guarantee that the value
    // is valid for the duration of the thread execution.
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
    // Rust does not let the thread borrow the value automatically since it does
    // not know how long it will be valid for.

    // Message passing is used for ensuring safe concurrency where threads or
    // actors communicate by sending each other messages containing data.

    // MPSC -> multi-producer, single consumer
    use std::sync::mpsc;

    // Create a new channel with sender and receiver
    // Single channel can only send and recv a single type.
    let (tx, rx) = mpsc::channel();
    // Create a new sender by cloning
    let tx2: mpsc::Sender<String> = tx.clone();
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        let vals: Vec<String> = vec![String::from("hi"), String::from("from"), String::from("me")];
        for val in vals {
            if let Err(err) = tx.send(val) {
                println!("Error with {err}");
            }
            thread::sleep(Duration::from_secs(1));
            // Trying to use val here will cause an error since its ownership has
            // been transferred over through `send`
        }
    });

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("this"),
            String::from("is"),
            String::from("tx2"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Call `recv` explicitly
    let received = rx.recv().unwrap();
    println!("Got {received}");
    // Or use iterators
    for received in rx {
        println!("Got: {received}");
    }

    handle.join().unwrap();
}
