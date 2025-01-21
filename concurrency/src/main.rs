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

    // Shared-state concurrency: multipe threads accessing same shared data.
    use std::sync::{Arc, Mutex};

    let m: Mutex<i32> = Mutex::new(5);
    {
        // Try to acquire lock from Mutex; this will block
        let mut num: std::sync::MutexGuard<'_, i32> = m.lock().unwrap();
        *num = 6;
    } // Lock is released automatically as `num` / MutexGuard goes out of scope.
    println!("m = {m:?}");

    // Multiple ownership of shared data.
    // Use Arc<> instead of Rc<>, Arc = atomic Rc to ensure thread-safety.
    // Mutext provides interior mutability: we can get mutable ref from the immutable instance
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result is {}", *counter.lock().unwrap());

    // `Sync` and `Send` traits

    // Send marker trait indicates that ownership of values of the type
    // implementing `Send` can be _transferred_ between threads.

    // `Sync` marker trait indicates that it is safe for the type to be
    // _referenced_ from multiple threads.
    // Similar to "thread-safety"

    /*
    * The smart pointer Rc<T> is also neither Send nor Sync since it's designed for single-thread use.
    * The RefCell<T> type (which we talked about in Chapter 15) and the family of related Cell<T> types are Send (if T: Send), but they are not Sync. A RefCell can be sent across a thread boundary, but not accessed concurrently because the implementation of borrow checking that RefCell<T> does at runtime is not thread-safe.
    * The smart pointer Mutex<T> is Send and Sync, and can be used to share access with multiple threads as you saw in the “Sharing a Mutex<T> Between Multiple Threads” section.
    * The type MutexGuard<'a, T> that is returned by Mutex::lock is Sync (if T: Sync) but not Send. It is specifically not Send because some platforms mandate that mutexes are unlocked by the same thread that locked them.
    */

    // Implementing `Send` and `Sync` manually is unsafe.
    // As marker traits, they do not have any methods to implement.

}
