pub trait Peek<T> {
    fn peek(&self) -> Option<&T>;
}

impl<T> Peek<T> for Vec<T> {
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self[self.len() - 1])
        }
    }
}
