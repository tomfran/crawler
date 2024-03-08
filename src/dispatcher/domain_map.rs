use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub struct DomainUrlsMap {
    map: HashMap<String, VecDeque<String>>,
}

impl DomainUrlsMap {
    pub fn add(&mut self, base_domain: String, sub_domain: String) {
        self.map
            .entry(base_domain)
            .or_default()
            .push_back(sub_domain);
    }

    pub fn contains(&self, base_domain: &String) -> bool {
        self.map.contains_key(base_domain)
    }

    pub fn get_next_url(&mut self, base_domain: &str) -> Option<String> {
        self.map.get_mut(base_domain).and_then(VecDeque::pop_front)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut d = DomainUrlsMap::default();

        d.add("test".to_string(), "sub_test".to_string());

        assert_eq!(d.get_next_url("test"), Some("sub_test".to_string()));
        assert!(d.get_next_url("test").is_none());
        assert!(d.get_next_url("new").is_none());

        assert!(!d.contains(&"random".to_string()));
    }
}
