use std::sync::Arc;
use std::thread;

use crate::dispatcher::Dispatcher;
use crate::worker::Worker;

pub struct Crawler {
    dispatcher: Arc<Dispatcher>,
    workers: Vec<Arc<Worker>>,
}

impl Crawler {
    pub fn new(num_workers: usize) -> Crawler {
        let dispatcher = Arc::new(Dispatcher::default());

        let workers = (0..num_workers)
            .map(|id| Arc::new(Worker::new(id as u32, Arc::clone(&dispatcher))))
            .collect();

        Crawler {
            dispatcher: Arc::clone(&dispatcher),
            workers,
        }
    }

    pub fn set_seed(&self, seed: &[String]) {
        self.dispatcher.add_urls(seed);
    }

    pub fn start(&self) {
        let mut threads = Vec::new();

        let dispatcher = self.dispatcher.clone();
        let t = thread::spawn(move || dispatcher.sieve_to_heap_loop());

        threads.push(t);

        for w in &self.workers {
            let worker = w.clone();
            let t = thread::spawn(move || worker.fetch_loop());
            threads.push(t);
        }

        for t in threads {
            t.join().unwrap();
        }
    }
}
