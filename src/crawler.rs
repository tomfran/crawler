use std::sync::{Arc, Mutex};
use std::thread;

use crate::{sieve::Sieve, worker::Worker};

pub struct Crawler {
    sieve: Arc<Mutex<Sieve>>,
    workers: Vec<Arc<Worker>>,
}

impl Crawler {
    pub fn new(num_workers: usize, max_visited_urls: usize) -> Crawler {
        let sieve = Arc::new(Mutex::new(Sieve::new(max_visited_urls)));

        let workers = (0..num_workers)
            .map(|id| Arc::new(Worker::new(id as u32, Arc::clone(&sieve))))
            .collect();

        Crawler {
            sieve: Arc::clone(&sieve),
            workers,
        }
    }

    pub fn set_seed(&self, seed: Vec<String>) {
        let sieve = Arc::clone(&self.sieve);
        let mut locked_sieve = sieve.lock().unwrap();

        for url in seed {
            locked_sieve.push(url);
        }
    }

    pub fn start(&self) {
        let mut threads = Vec::new();

        for w in &self.workers {
            let local_w = w.clone();
            let t = thread::spawn(move || local_w.start());
            threads.push(t);
        }

        for t in threads {
            t.join().unwrap();
        }
    }
}
