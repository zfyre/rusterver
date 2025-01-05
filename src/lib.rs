
use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(num_threads: usize) -> ThreadPool {
        assert!(num_threads > 0);

        let (sender, receiver) = 
                mpsc::channel();
        
        let receiver = 
                Arc::new(Mutex::new(receiver));
        
        let mut workers = 
                Vec::with_capacity(num_threads);
        
        for id in 0..num_threads {
            // create threads
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
            ));
        }

        Self {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static 
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Drop method is the way the variables are removed from the memory!
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // worker.thread.join().unwrap();
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
            .lock() // It's thread lock for accessing the variables in a different thread
            .unwrap()
            .recv()
            .unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job()
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });
        Self {
            id,
            thread: Some(thread)
        }
    }
}