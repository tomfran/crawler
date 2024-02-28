use crate::{fetch::Fetcher, sieve::Sieve};
use log::{info, warn};
use std::{
    sync::{Arc, Mutex},
    thread, time,
};

pub struct Worker {
    id: u32,
    fetcher: Fetcher,
    sieve: Arc<Mutex<Sieve>>,
}

impl Worker {
    pub fn new(id: u32, sieve: Arc<Mutex<Sieve>>) -> Worker {
        Worker {
            id,
            fetcher: Fetcher::default(),
            sieve,
        }
    }

    pub fn start(&self) {
        let initial_sleep_time = time::Duration::from_millis(10);
        let mut current_sleep_time = initial_sleep_time;

        for _ in 0..100 {
            let url = self.get_url();

            match url {
                Some(u) => self.fetch(u),
                None => {
                    warn!(
                        "[Worker {}] - Sleeping for {} ms",
                        self.id,
                        current_sleep_time.as_millis()
                    );
                    thread::sleep(current_sleep_time);
                    current_sleep_time *= 2
                }
            }
        }
    }

    fn get_url(&self) -> Option<String> {
        let sieve = Arc::clone(&self.sieve);
        let url = sieve.lock().unwrap().pop();
        url
    }

    fn fetch(&self, url: String) {
        let optional_webpage = self.fetcher.fetch(&url);
        if optional_webpage.is_none() {
            return;
        }

        let webpage = optional_webpage.unwrap();
        info!("[Worker {}] - Fetched {}", self.id, webpage.url);

        let sieve = Arc::clone(&self.sieve);
        let mut locked_sieve = sieve.lock().unwrap();

        for url in webpage.links {
            locked_sieve.push(url);
        }
    }
}
