use std::ops::{Sub, Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn parse(pnt: &str) -> Self {
        pnt.split_once(",").map(|p|Point(p.0.parse().unwrap(), p.1.parse().unwrap())).unwrap()
    }

    pub fn singlify(&self) -> Self {
        Self(self.0.signum(), self.1.signum())
    }

    pub fn points_between(&self, other: &Self) -> PointIter {
        PointIter { from: *self, to: *other, first: true }
    }
}

pub struct PointIter {
    from: Point,
    to: Point,
    first: bool
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some(self.from)
        } else if &self.from == &self.to {
            None
        } else {
            self.from = self.from + (self.to - self.from).singlify();
            Some(self.from)
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}