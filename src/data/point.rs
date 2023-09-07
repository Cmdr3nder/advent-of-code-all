use std::default::Default;
use std::iter::Iterator;
use std::ops::{Add, AddAssign, Sub};

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

impl<T: Add<Output = T>> Add<(T, T)> for Point2D<T> {
    type Output = Point2D<T>;

    fn add(self, vector: (T, T)) -> Point2D<T> {
        Point2D::new(self.x + vector.0, self.y + vector.1)
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

impl<T: Sub<Output = T> + PartialOrd + Copy> Point2D<T>
where
    usize: TryFrom<T>,
{
    pub fn to_index(&self, bounds: &(Point2D<T>, Point2D<T>)) -> Option<usize> {
        let (low_x, low_y, high_x, high_y) =
            match (bounds.0.x <= bounds.1.x, bounds.0.y <= bounds.1.y) {
                (true, true) => (bounds.0.x, bounds.0.y, bounds.1.x, bounds.1.y),
                (true, false) => (bounds.0.x, bounds.1.y, bounds.1.x, bounds.0.y),
                (false, true) => (bounds.1.x, bounds.0.y, bounds.0.x, bounds.1.y),
                (false, false) => (bounds.1.x, bounds.1.y, bounds.0.x, bounds.0.y),
            };
        if self.x < low_x || self.x > high_x || self.y < low_y || self.y > high_y {
            None
        } else {
            let width: usize = (high_x - low_x)
                .try_into()
                .ok()
                .expect("width should be convertable to usize"); // Should be safe since the subtraction will bring any negatives into the positive realm
            let width = width + 1;
            let x: usize = (self.x - low_x)
                .try_into()
                .ok()
                .expect("x should be convertable to usize");
            let y: usize = (self.y - low_y)
                .try_into()
                .ok()
                .expect("y should be convertable to usize");
            Some(y * width + x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_index() {
        let data: Vec<((i32, i32), ((i32, i32), (i32, i32)), Option<usize>)> = vec![
            ((1, 2), ((0, 0), (2, 2)), Some(7)),
            ((0, 0), ((0, 0), (2, 2)), Some(0)),
            ((0, 1), ((0, 0), (2, 2)), Some(3)),
            ((0, 2), ((0, 0), (2, 2)), Some(6)),
            ((1, 0), ((0, 0), (2, 2)), Some(1)),
            ((-1, 0), ((0, 0), (2, 2)), None),
            ((0, -1), ((0, 0), (2, 2)), None),
            ((-1, -1), ((0, 0), (2, 2)), None),
            ((3, -1), ((0, 0), (2, 2)), None),
            ((-1, 3), ((0, 0), (2, 2)), None),
            ((5, 3), ((0, 0), (2, 2)), None),
            ((1, -1), ((-2, -3), (4, 5)), Some(17)),
            ((-2, -3), ((-2, -3), (4, 5)), Some(0)),
            ((-2, 3), ((-2, -3), (4, 5)), Some(42)),
            ((2, 5), ((-2, -3), (4, 5)), Some(60)),
            ((3, 1), ((-2, -3), (4, 5)), Some(33)),
        ];
        for (point, dimensions, expected) in data {
            let point: Point2D<i32> = Point2D::new(point.0, point.1);
            let dimensions: (Point2D<i32>, Point2D<i32>) = (
                Point2D::new(dimensions.0 .0, dimensions.0 .1),
                Point2D::new(dimensions.1 .0, dimensions.1 .1),
            );
            assert_eq!(point.to_index(&dimensions), expected);
        }
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
