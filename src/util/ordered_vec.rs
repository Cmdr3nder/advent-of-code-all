#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OrderedVec<T: Ord> {
    items: Vec<T>,
}

impl<T: std::hash::Hash + Ord> std::hash::Hash for OrderedVec<T> {
    fn hash<H>(&self, h: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.items.hash(h)
    }
}

impl<T: Ord> Default for OrderedVec<T> {
    fn default() -> Self {
        OrderedVec::new()
    }
}

impl<T, Idx> std::ops::Index<Idx> for OrderedVec<T>
where
    T: Ord,
    Idx: std::slice::SliceIndex<[T]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.items[index]
    }
}

impl<T: Ord> OrderedVec<T> {
    pub fn new() -> Self {
        OrderedVec { items: Vec::new() }
    }

    /*pub fn from_unsorted(mut items: Vec<T>) -> Self {
        items.sort_unstable();
        OrderedVec { items }
    }*/

    pub fn from_one(item: T) -> Self {
        let mut vec = OrderedVec::new();
        vec.items.push(item);
        vec
    }

    pub fn insert(&mut self, item: T) -> usize {
        let index = match self.items.binary_search(&item) {
            Ok(i) | Err(i) => i,
        };
        self.items.insert(index, item);
        index
    }

    pub fn push(&mut self, item: T) -> usize {
        if let Some(last) = self.items.last() {
            let cmp = item.cmp(last);
            if cmp == std::cmp::Ordering::Greater || cmp == std::cmp::Ordering::Equal {
                self.items.push(item);
                self.items.len() - 1
            } else {
                self.insert(item)
            }
        } else {
            self.items.push(item);
            0
        }
    }

    /*pub fn remove_item(&mut self, item: &T) -> Option<T> {
        match self.items.binary_search(item) {
            Ok(i) => Some(self.items.remove(i)),
            Err(_) => None
        }
    }*/

    pub fn remove(&mut self, index: usize) -> T {
        self.items.remove(index)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /*pub fn clear(&mut self) {
        self.items.clear()
    }*/

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let mut vec = OrderedVec::new();
        vec.push(5);
        vec.push(1);
        vec.insert(2);
        assert_eq!(&[1, 2, 5], vec.as_slice());
    }
}
