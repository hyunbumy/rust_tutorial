use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// Must use Box<dyn ...> to accept any closures
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Make blocking channel receive with specified number of threads.
        let (tx, rx) = mpsc::channel();
        // Create a ref-counting pointers to the _same_ receiver.
        // We need to guard it with a mutex for thread-safety.
        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);
        for ind in 0..size {
            let receiver = Arc::clone(&receiver);
            // We can't use `while let` since the Mutex unlocks as it goes out of scope,
            // but with `while let` the RHS does not go out of scope until the end of the block.
            // OTOH, with `let` the RHS goes out of scope at the end of its statement.
            workers.push(thread::spawn(move || loop {
                let job: Job = receiver.lock().unwrap().recv().unwrap();
                println!("Got a job from {ind}");

                job();
            }));
        }

        ThreadPool {
            workers,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, func: F)
    where
        // We need `Send` to transfer closure from one thread to another
        // We need `'static` since we don't know how long the thread will take to execute.
        F: FnOnce() + Send +'static,
    {
        // Send func as fast as we can
        let job = Box::new(func);
        self.sender.send(job).unwrap();
    }
}
