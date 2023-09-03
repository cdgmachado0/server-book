use std::{
    thread::{self, JoinHandle},
    sync::{mpsc, Arc, Mutex}
};

/// Struct for implementing a multi-threaded pooling strategy.
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}


type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Error
    ///
    /// The `build` function will return a `PoolCreationError` struct if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }   

            Ok(ThreadPool { 
                workers, 
                sender: Some(sender) 
            })
        } else {
            Err(PoolCreationError)
        }
    }

    /// Transmits a received job to the pool where all threads are, so the job gets ran.
    /// 
    /// `f` is the job to run acquired by a closure.
    /// 
    /// # Panics
    /// 
    /// Functions panics if the sender has been dropped or the channel has hung.
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap_or_else(|_| panic!("The receiver is disconnected"));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                match thread.join() {
                    Err(e) => {
                        eprintln!("Thread {} couldn't finish: {:?}.", worker.id, e);
                        continue;
                    },
                    _ => {}
                }
            }
        }
    }
}


#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move|| loop {
            let message = receiver.lock().unwrap().recv(); 

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing...");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down...");
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}

/// Error struct for when `ThreadPool` can't be created.
#[derive(Debug)]
pub struct PoolCreationError; 

impl PoolCreationError {
    pub fn throw(&self) -> String {
        String::from("Thread pool not created")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn create_pool(num: usize) -> Result<ThreadPool, PoolCreationError> {
        ThreadPool::build(num)
    }

    #[test]
    fn success_pool_create() {
        let pool = create_pool(4).unwrap();
        assert_eq!(pool.workers.len(), 4);

        pool.workers.iter()
            .for_each(|worker| {
                let thread = &worker.thread;
                assert_eq!(thread.as_ref().unwrap().is_finished(), false);
            });
    }

    #[test]
    fn fail_pool_create() {
        let err = create_pool(0).unwrap_err().throw();
        assert_eq!(err, "Thread pool not created");
    }

    #[test]
    fn success_executes_job() {
        let pool = create_pool(4).unwrap();
        let job = || println!("hello world\n");
        pool.execute(job);
    }

    #[test]
    #[should_panic]
    fn fail_panics_at_job() {
        let pool = create_pool(4).unwrap();
        let job = || panic!("Panic expected");
        pool.execute(job);
    }
}