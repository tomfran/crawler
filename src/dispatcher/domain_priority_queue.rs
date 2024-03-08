use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    time::{Duration, SystemTime},
};

const POLITENESS_DELAY: Duration = Duration::from_millis(500);

#[derive(Eq, Debug)]
pub struct DomainTimestampPair {
    domain: String,
    visit_timestamp: SystemTime,
}

impl DomainTimestampPair {
    pub fn new(domain: String) -> DomainTimestampPair {
        DomainTimestampPair {
            domain,
            visit_timestamp: SystemTime::now(),
        }
    }

    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }

    pub fn set_next_visit_timestamp(&mut self) {
        self.visit_timestamp = SystemTime::now() + POLITENESS_DELAY;
    }

    pub fn is_visitable(&self) -> bool {
        self.visit_timestamp <= SystemTime::now()
    }
}

impl PartialEq for DomainTimestampPair {
    fn eq(&self, other: &Self) -> bool {
        self.visit_timestamp == other.visit_timestamp
    }
}

impl PartialOrd for DomainTimestampPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DomainTimestampPair {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .visit_timestamp
            .partial_cmp(&self.visit_timestamp)
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(Default)]
pub struct DomainPriorityQueue {
    heap: BinaryHeap<DomainTimestampPair>,
}

impl DomainPriorityQueue {
    pub fn add_domain(&mut self, domain: String) {
        self.heap.push(DomainTimestampPair::new(domain));
    }

    pub fn add_pair(&mut self, element: DomainTimestampPair) {
        self.heap.push(element);
    }

    pub fn pop(&mut self) -> Option<DomainTimestampPair> {
        self.heap.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut queue = DomainPriorityQueue::default();

        let now = SystemTime::now();
        queue.add_domain("first".to_string());
        queue.add_domain("second".to_string());

        queue.add_pair(DomainTimestampPair {
            domain: "manual".to_string(),
            visit_timestamp: now,
        });

        assert_eq!(queue.pop().unwrap().domain, "manual");
        assert_eq!(queue.pop().unwrap().domain, "first");
        assert_eq!(queue.pop().unwrap().domain, "second");
    }
}
