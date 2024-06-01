
use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};


type Job = Box<dyn FnOnce() + Send + 'static>;


enum Message {
    NewJob(Job),
    Terminate,
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}


// implement threadpool
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}


// implement drop trait for threadpool
impl Drop for ThreadPool {
    fn drop(&mut self) {

        println!("Sending terminate message to all workers.");
        
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


// worker thread
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}


// implementation of worker thread
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}