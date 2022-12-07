use crate::line_algorithms;

#[derive(Clone, Copy, Debug)]
pub struct Point(pub i32, pub i32);

#[derive(Clone, Copy, Debug)]
pub struct Line(pub Point, pub Point);

#[derive(Debug)]
pub struct Wire(pub Vec<Line>);

impl Line {
    pub fn contains(&self, point: &Point) -> bool {
        if point.0 <= self.0.0.max(self.1.0) // to.x <= max(from.x, point.x)
        && point.0 >= self.0.0.min(self.1.0) // to.x >= min(from.x, point.x)
        && point.1 <= self.0.1.max(self.1.1) // to.y <= max(from.y, point.y)
        && point.1 >= self.0.1.min(self.1.1)
        // to.y >= min(from.y, point.y)
        {
            true
        } else {
            false
        }
    }

    fn intersects_with(&self, other: &Line) -> bool {
        let o1 = Point::orientation(&self.0, &self.1, &other.0);
        let o2 = Point::orientation(&self.0, &self.1, &other.1);
        let o3 = Point::orientation(&other.0, &other.1, &self.0);
        let o4 = Point::orientation(&other.0, &other.1, &self.1);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        if o1 == Orientation::Collinear && self.contains(&other.0) {
            return true;
        }

        if o2 == Orientation::Collinear && self.contains(&other.1) {
            return true;
        }

        if o3 == Orientation::Collinear && other.contains(&self.0) {
            return true;
        }

        if o4 == Orientation::Collinear && other.contains(&self.1) {
            return true;
        }

        false
    }

    // Get intersection line or point
    pub fn intersect(&self, line: &Self) -> Option<Line> {
        if self.intersects_with(line) && line.intersects_with(self) {
            Some(line_algorithms::get_intersection(self, line))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Orientation {
    Collinear,
    Clockwise,
    Counterclockwise,
}

impl Point {
    pub fn distance(&self, other: &Point) -> i32 {
        (other.0 - self.0).abs() + (other.1 - self.1).abs()
    }

    pub fn orientation(p1: &Point, p2: &Point, p3: &Point) -> Orientation {
        match (p2.1 - p1.1) * (p3.0 - p2.0) - (p2.0 - p1.0) * (p3.1 - p2.1) {
            0 => Orientation::Collinear,
            n if n.is_positive() => Orientation::Clockwise,
            _ => Orientation::Counterclockwise,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn can_check_if_point_is_on_line() {
        let line = Line(Point(1, 3), Point(1, 5));
        assert!(line.contains(&Point(1, 3)));
        assert!(line.contains(&Point(1, 4)));
        assert!(line.contains(&Point(1, 5)));
        assert!(!line.contains(&Point(1, 6)));
        assert!(!line.contains(&Point(2, 3)));
    }

    #[test]
    fn can_check_point_on_diagonal_line() {
        let line = Line(Point(2, 2), Point(4, 4));
        assert!(line.contains(&Point(3, 3)));
    }

    #[test]
    fn can_check_intersection_horizontal() {
        let line1 = Line(Point(1, 3), Point(1, 5));
        let line2 = Line(Point(5, 4), Point(0, 4));
        assert!(line1.intersects_with(&line2));
    }

    #[test]
    fn can_check_intersection_vertical() {
        let line1 = Line(Point(5, 2), Point(0, 2));
        let line2 = Line(Point(3, 1), Point(3, 3));
        let line3 = Line(Point(5, 2), Point(5, 3));

        assert!(line1.intersects_with(&line2));
        assert!(line1.intersects_with(&line3));
    }
}
