mod domain_map;
mod domain_priority_queue;

use self::{domain_map::DomainUrlsMap, domain_priority_queue::DomainPriorityQueue};
use crate::{
    dispatcher::domain_priority_queue::DomainTimestampPair, sieve::Sieve, url_utils::extract_domain,
};
use log::debug;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const INITIAL_SLEEP_TIME: Duration = Duration::from_millis(500);

#[derive(Default)]
pub struct Dispatcher {
    sieve: Arc<Mutex<Sieve>>,
    queue: Arc<Mutex<DomainPriorityQueue>>,
    map: Arc<Mutex<DomainUrlsMap>>,
}

impl Dispatcher {
    pub fn add_urls(&self, urls: &[String]) {
        let mut sieve = self.sieve.lock().unwrap();

        for url in urls {
            sieve.push(url.clone());
        }
    }

    pub fn get_next_url(&self) -> Option<String> {
        let mut queue = self.queue.lock().unwrap();
        let mut head = queue.pop().filter(DomainTimestampPair::is_visitable)?;

        let mut map = self.map.lock().unwrap();
        let url = map.get_next_url(&head.get_domain())?;

        head.set_next_visit_timestamp();
        queue.add_pair(head);

        Some(url)
    }

    pub fn sieve_to_heap_loop(&self) {
        let initial_sleep_time = INITIAL_SLEEP_TIME;
        let mut current_sleep_time = initial_sleep_time;

        loop {
            if let Some(url) = self.get_next_sieve_url() {
                self.add_to_queue(url);
                current_sleep_time = initial_sleep_time;
            } else {
                debug!(
                    "[Dispatcher] - Empty Sieve, sleeping for {} ms",
                    current_sleep_time.as_millis(),
                );
                thread::sleep(current_sleep_time);
                current_sleep_time *= 2;
            }
        }
    }

    fn get_next_sieve_url(&self) -> Option<String> {
        self.sieve.lock().unwrap().pop()
    }

    fn add_to_queue(&self, url: String) {
        let mut queue = self.queue.lock().unwrap();
        let mut map = self.map.lock().unwrap();

        let optional_base_domain = extract_domain(&url);
        if optional_base_domain.is_none() {
            return;
        }

        let base_domain = optional_base_domain.unwrap();

        if !map.contains(&base_domain) {
            queue.add_domain(base_domain.clone());
        }
        map.add(base_domain, url);
    }
}
