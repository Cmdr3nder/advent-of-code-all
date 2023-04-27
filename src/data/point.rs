use std::default::Default;
use std::iter::Iterator;
use std::ops::{Add, AddAssign, Mul};

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

impl<T> Point2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Point2D { x, y }
    }
}

pub struct Point2DIterator<T> {
    next_x: T,
    next_y: T,
    first_x: T,
    last_x: T,
    last_y: T,
}

impl<T: AddAssign + Copy + From<u8> + PartialOrd> Iterator for Point2DIterator<T> {
    type Item = Point2D<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y > self.last_y {
            return None;
        }
        let res = Some(Point2D::new(self.next_x, self.next_y));
        self.next_x += 1.into();
        if self.next_x > self.last_x {
            self.next_x = self.first_x;
            self.next_y += 1.into();
        }
        res
    }
}

impl<T: PartialOrd + Copy> Point2D<T> {
    pub fn iter_to(&self, to: &Self) -> Point2DIterator<T> {
        match (self.x <= to.x, self.y <= to.y) {
            (true, true) => Point2DIterator {
                next_x: self.x,
                first_x: self.x,
                next_y: self.y,
                last_x: to.x,
                last_y: to.y,
            },
            (true, false) => Point2DIterator {
                next_x: self.x,
                first_x: self.x,
                next_y: to.y,
                last_x: to.x,
                last_y: self.y,
            },
            (false, true) => Point2DIterator {
                next_x: to.x,
                first_x: to.x,
                next_y: self.y,
                last_x: self.x,
                last_y: to.y,
            },
            (false, false) => Point2DIterator {
                next_x: to.x,
                first_x: to.x,
                next_y: to.y,
                last_x: self.x,
                last_y: self.y,
            },
        }
    }
}

impl<T: Mul + Add> Point2D<T> {
    pub fn to_index(&self, bounds: &(Point2D, Point2D)) -> usize {
        let (low_x, low_y, high_x, high_y) = match (bounds.0.x <= bounds.1.x, bounds.0.y <= bounds.1.y) {
            (true, true) => (bounds.0.x, bounds.0.y, bounds.1.x, bounds.1.y),
            (true, false) => (bounds.0.x, bounds.1.y, bounds.1.x, bounds.0.y),
            (false, true) => (bounds.1.x, bounds.0.y, bounds.0.x, bounds.1.y),
            (false, false) => (bounds.1.x, bounds.1.y, bounds.0.x, bounds.0.y),
        };
        (self.y * dimensions.x) + self.x
    }
}

/*
impl<T> Point2D<T> {
    pub fn from_index(index: usize, dimensions: &Self) -> Self {
        let x = index % dimensions.x;
        let y = index / dimensions.x;
        Point2D { x, y }
    }
}*/
