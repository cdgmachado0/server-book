use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        ThreadPool
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            // let mut threads = Vec::new();

            Ok(ThreadPool)
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


pub struct PoolCreationError; 

impl PoolCreationError {
    pub fn throw(&self) -> String {
        String::from("Thread pool not created")
    }
}