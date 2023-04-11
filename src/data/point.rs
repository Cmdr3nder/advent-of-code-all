use std::ops::AddAssign;

#[derive(Clone, Copy, Eq, Hash, Debug, Default, PartialEq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: AddAssign> AddAssign<(T, T)> for Point2D<T> {
    fn add_assign(&mut self, vector: (T, T)) {
        self.x += vector.0;
        self.y += vector.1;
    }
}
