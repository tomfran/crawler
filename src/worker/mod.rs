pub mod fetch;
pub mod simhash;

use self::fetch::Fetcher;
use crate::dispatcher::Dispatcher;
use log::{debug, info};
use std::{sync::Arc, thread, time::Duration};

const INITIAL_SLEEP_TIME: Duration = Duration::from_millis(10);

pub struct Worker {
    id: u32,
    fetcher: Fetcher,
    dispatcher: Arc<Dispatcher>,
}

impl Worker {
    pub fn new(id: u32, dispatcher: Arc<Dispatcher>) -> Worker {
        Worker {
            id,
            fetcher: Fetcher::default(),
            dispatcher,
        }
    }

    pub fn fetch_loop(&self) {
        let initial_sleep_time = INITIAL_SLEEP_TIME;
        let mut current_sleep_time = initial_sleep_time;

        loop {
            if let Some(u) = self.get_url() {
                self.fetch(&u);
                current_sleep_time = initial_sleep_time;
            } else {
                debug!(
                    "[Worker {}] - Empty dispatcher, sleeping for {} ms",
                    self.id,
                    current_sleep_time.as_millis()
                );
                thread::sleep(current_sleep_time);
                current_sleep_time *= 2;
            }
        }
    }

    fn get_url(&self) -> Option<String> {
        self.dispatcher.get_next_url()
    }

    fn fetch(&self, url: &str) {
        let optional_webpage = self.fetcher.fetch(url);
        if optional_webpage.is_none() {
            return;
        }

        let webpage = optional_webpage.unwrap();
        info!("[Worker {}] - Fetched {}", self.id, webpage.url);

        self.dispatcher.add_urls(&webpage.links);
    }
}
