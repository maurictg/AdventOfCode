use std::ops::Add;

use crate::point::Point;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    // Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            //Direction::Up(n) => Point(self.0, self.1 + n),
            Direction::Down(n) => Point(self.0, self.1 - n),
            Direction::Left(n) => Point(self.0 - n, self.1),
            Direction::Right(n) => Point(self.0 + n, self.1),
        }
    }
}