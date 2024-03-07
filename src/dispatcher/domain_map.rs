use std::collections::{HashMap, LinkedList};

#[derive(Default)]
pub struct DomainUrlsMap {
    map: HashMap<String, LinkedList<String>>,
}

impl DomainUrlsMap {
    pub fn add(&mut self, base_domain: String, sub_domain: String) {
        todo!()
    }

    pub fn get(&mut self, base_domain: String) -> Option<String> {
        todo!()
    }
}
