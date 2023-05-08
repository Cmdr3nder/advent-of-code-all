use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;

pub struct PriorityQueue<T, P> {
    heap: BinaryHeap<PriorityQueueItem<T, P>>,
}

struct PriorityQueueItem<T, P> {
    value: T,
    priority: P,
}

impl<T, P> PartialEq for PriorityQueueItem<T, P>
where
    T: PartialEq,
{
    fn eq(&self, other: &PriorityQueueItem<T, P>) -> bool {
        self.value == other.value
    }
}

impl<T, P> PartialOrd for PriorityQueueItem<T, P>
where
    T: PartialEq,
    P: PartialOrd,
{
    fn partial_cmp(&self, other: &PriorityQueueItem<T, P>) -> Option<Ordering> {
        Some(if self.priority < other.priority {
            Ordering::Less
        } else if self.priority > other.priority {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl<T, P> Ord for PriorityQueueItem<T, P>
where
    T: PartialEq,
    P: PartialOrd,
{
    fn cmp(&self, other: &PriorityQueueItem<T, P>) -> Ordering {
        if self.priority < other.priority {
            Ordering::Less
        } else if self.priority > other.priority {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<T, P> Eq for PriorityQueueItem<T, P> where T: PartialEq {}

impl<T, P> PriorityQueueItem<T, P> {
    fn new(value: T, priority: P) -> Self {
        PriorityQueueItem { value, priority }
    }
}

impl<T, P> PriorityQueue<T, P>
where
    T: PartialEq,
    P: PartialOrd,
{
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, value: T, priority: P) {
        let item = PriorityQueueItem::new(value, priority);
        self.heap.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.value)
    }
}
