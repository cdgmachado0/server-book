use std::{
    thread::{self, JoinHandle},
    sync::mpsc,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `build` function will return a `PoolCreationError` struct if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id));
            }   

            Ok(ThreadPool { workers, sender })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {

    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}


pub struct PoolCreationError; 

impl PoolCreationError {
    pub fn throw(&self) -> String {
        String::from("Thread pool not created")
    }
}