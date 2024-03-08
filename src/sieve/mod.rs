pub mod bloom;

use self::bloom::Filter;
use std::collections::VecDeque;

#[derive(Default)]
pub struct Sieve {
    filter: Filter,
    urls: VecDeque<String>,
}

impl Sieve {
    pub fn new(expected_urls_num: usize) -> Sieve {
        Sieve {
            filter: Filter::new(expected_urls_num, 0.01),
            urls: VecDeque::new(),
        }
    }

    pub fn push(&mut self, url: String) {
        let url_bytes = url.as_bytes();

        if self.filter.contains(url_bytes) {
            return;
        }

        self.filter.add(url_bytes);
        self.urls.push_back(url);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.urls.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_push_pop() {
        let mut s = Sieve::new(10);

        s.push("1".to_string());
        s.push("2".to_string());
        s.push("1".to_string());

        assert_eq!(s.pop().unwrap(), "1");
        assert_eq!(s.pop().unwrap(), "2");
        assert!(s.pop().is_none());
    }
}
