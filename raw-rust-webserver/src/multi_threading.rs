
use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads on the pool
    /// 
    /// # Panics
    /// 
    /// The `new` functions will panic if size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // raise error when size is 0
        assert!(size > 0);

        // create channel
        let (sender, receiver) = mpsc::channel();

        // creating thread save smart pointer
        let receiver = Arc::new(Mutex::new(receiver));


        // create workers list
        let mut workers = Vec::with_capacity(size);

        // spawn worker
        for id in 0..size {
            // create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        // create and return threadpools
        ThreadPool {
            workers,
            sender
        }
    }

    // Execute method
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
        
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker {
            id,
            thread
        }
    }
}