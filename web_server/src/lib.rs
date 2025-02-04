use crossbeam_channel;
use std::thread;

// Must use Box<dyn ...> to accept any closures
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Option<thread::JoinHandle<()>>>,
    sender: crossbeam_channel::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Make blocking channel receive with specified number of threads.
        let (tx, rx) = crossbeam_channel::bounded(1);
        // Create a ref-counting pointers to the _same_ receiver.
        // We need to guard it with a mutex for thread-safety.
        // let receiver = Arc::new(Mutex::new(rx));

        // Alternatively, use a mpmc so that the rx can be cloned.
        let mut workers = Vec::with_capacity(size);
        for ind in 0..size {
            let receiver = rx.clone();
            // We can't use `while let` since the Mutex unlocks as it goes out of scope,
            // but with `while let` the RHS does not go out of scope until the end of the block.
            // OTOH, with `let` the RHS goes out of scope at the end of its statement.
            workers.push(Option::Some(thread::spawn(move || loop {
                let job: Job = receiver.recv().unwrap();
                println!("Got a job from {ind}");

                job();
            })));
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
        F: FnOnce() + Send + 'static,
    {
        // Send func as fast as we can
        let job = Box::new(func);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down workers");
        for worker in &mut self.workers {
          if let Some(handle) = worker.take() {
            handle.join().unwrap();
          }
        }
    }
}
