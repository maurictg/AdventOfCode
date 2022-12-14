use std::{fs, ops::{Add, Sub}, iter::repeat, collections::HashSet};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Direction {
    pub fn n(&self) -> i32 {
        match self {
            Direction::Up(n) => *n,
            Direction::Down(n) => *n,
            Direction::Left(n) => *n,
            Direction::Right(n) => *n,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Point {
    pub fn singlify(&self) -> Self {
        Self(self.0.signum(), self.1.signum())
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up(n) => Point(self.0, self.1 + n),
            Direction::Down(n) => Point(self.0, self.1 - n),
            Direction::Left(n) => Point(self.0 - n, self.1),
            Direction::Right(n) => Point(self.0 + n, self.1),
        }
    }
}

impl Direction {
    pub fn new(str: &str) -> Self {
        let res = str.split_once(" ").unwrap();
        let n: i32 = res.1.parse().unwrap();
        match res.0 {
            "U" => Direction::Up(n),
            "D" => Direction::Down(n),
            "L" => Direction::Left(n),
            "R" => Direction::Right(n),
            _ => unreachable!("Unknown direction: {}", res.0)
        }
    }
}

const KNOTS: usize = 10;

fn main() {
    let directions: Vec<Direction> = fs::read_to_string("input.txt").unwrap().lines().map(|l|Direction::new(l)).collect();
    
    let mut head = Point(0,0);
    let mut tail = Point(0,0);

    // Part 1
    let mut reported = HashSet::new();
    reported.insert(Point(0,0));

    for d in directions.iter() {
        head = head + *d;
        tail = follow(&tail, &head, Some(&mut reported));
    }

    println!("Part 1: {}", reported.len());

    // Part 2: does NOT work as expected. But works for the examples
    let mut reported = HashSet::new();
    reported.insert(Point(0,0));

    let mut knots: Vec<Point> = repeat(Point(0,0)).take(KNOTS).collect();

    for d in directions.iter() {
        knots[0] = knots[0] + *d;
        for _ in 0..d.n() {
            for k in 1..KNOTS {
                knots[k] = follow_1(&knots[k], &knots[k - 1], if k == KNOTS - 1 {
                    Some(&mut reported)
                } else { None });
            }
        }
    }

    println!("Part 2: {}", reported.len());
}

/*fn _follow(me: &Point, other: &Point, report: bool, reported: &mut HashSet<Point>) -> Option<Point> {

}*/

fn follow_1(me: &Point, other: &Point, mut reported_points: Option<&mut HashSet<Point>>) -> Point {
    let change = (*other - *me).singlify();
    let res = me.clone() + change;
    if &res != other {
        if reported_points.is_some() {
            reported_points.as_mut().unwrap().insert(res);
        }
        res
    } else {
        me.clone()
    }
}
 
fn follow(me: &Point, other: &Point, mut reported_points: Option<&mut HashSet<Point>>) -> Point {
    let mut new = me.clone();
    
    // Change on x or y axis
    while &new != other {
        let change = (*other - new).singlify();
        let n = new + change;
        if &n == other {
            break;
        } else {
            new = n;
            if reported_points.is_some() {
                reported_points.as_mut().unwrap().insert(new);
            }
        }
    }

    new
}