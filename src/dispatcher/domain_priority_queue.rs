use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    time::{Duration, SystemTime},
};

const POLITENESS_DELAY: Duration = Duration::new(2, 0);

#[derive(Eq, Debug)]
pub struct DomainPriorityPair {
    domain: String,
    visit_timestamp: SystemTime,
}

impl DomainPriorityPair {
    pub fn new(domain: String) -> DomainPriorityPair {
        DomainPriorityPair {
            domain,
            visit_timestamp: SystemTime::now(),
        }
    }

    pub fn set_next_visit(&mut self) {
        self.visit_timestamp += POLITENESS_DELAY
    }
}

impl PartialEq for DomainPriorityPair {
    fn eq(&self, other: &Self) -> bool {
        self.visit_timestamp == other.visit_timestamp
    }
}

impl PartialOrd for DomainPriorityPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DomainPriorityPair {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .visit_timestamp
            .partial_cmp(&self.visit_timestamp)
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(Default)]
pub struct DomainPriorityQueue {
    heap: BinaryHeap<DomainPriorityPair>,
}

impl DomainPriorityQueue {
    pub fn add_domain(&mut self, domain: String) {
        self.heap.push(DomainPriorityPair::new(domain));
    }

    pub fn add_pair(&mut self, element: DomainPriorityPair) {
        self.heap.push(element);
    }

    pub fn get(&mut self) -> Option<DomainPriorityPair> {
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

        queue.add_pair(DomainPriorityPair {
            domain: "manual".to_string(),
            visit_timestamp: now,
        });

        assert_eq!(queue.get().unwrap().domain, "manual");
        assert_eq!(queue.get().unwrap().domain, "first");
        assert_eq!(queue.get().unwrap().domain, "second");
    }
}
