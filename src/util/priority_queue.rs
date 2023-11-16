use crate::util::ordered_vec::OrderedVec;

#[derive(PartialEq, Eq)]
struct QueueItem<P, T>
where
    P: Ord,
    T: Eq,
{
    value: T,
    priority: P,
}

impl<P, T> QueueItem<P, T>
where
    P: Ord,
    T: Eq,
{
    fn new(value: T, priority: P) -> Self {
        QueueItem { value, priority }
    }
}

impl<P, T> PartialOrd for QueueItem<P, T>
where
    P: Ord,
    T: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<P, T> Ord for QueueItem<P, T>
where
    P: Ord,
    T: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

// Ordering::Greater means Greater Priority
pub struct PriorityQueue<P, T>
where
    P: Ord,
    T: Eq,
{
    que: OrderedVec<QueueItem<P, T>>,
}

impl<P, T> PriorityQueue<P, T>
where
    P: Ord,
    T: Eq,
{
    pub fn new() -> Self {
        PriorityQueue {
            que: OrderedVec::new(),
        }
    }

    pub fn push(&mut self, value: T, priority: P) {
        self.que.insert(QueueItem::new(value, priority));
    }

    pub fn pop(&mut self) -> Option<(T, P)> {
        self.que.pop().map(|item| (item.value, item.priority))
    }

    pub fn len(&self) -> usize {
        self.que.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = PriorityQueue::new();
        q.push("Hello", 5);
        q.push("World", 1);
        q.push("My", 3);
        assert_eq!(Some(("Hello", 5)), q.pop());
        q.push("Name", 66);
        q.push("Is", 66);
        assert_eq!(Some(("Name", 66)), q.pop()); // Earlier insert with same priority comes back first
        assert_eq!(Some(("Is", 66)), q.pop());
        assert_eq!(Some(("My", 3)), q.pop());
        assert_eq!(Some(("World", 1)), q.pop());
        assert_eq!(None, q.pop());
    }
}
