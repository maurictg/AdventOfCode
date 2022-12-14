use std::{fs, ops::{Add, Sub}, iter::repeat, collections::HashSet};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Direction {
    pub fn n_steps(&self) -> i32 {
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
        while let Some(tnew) = follow(&tail, &head, true, &mut reported) {
            tail = tnew;
        }
    }

    println!("Part 1: {}", reported.len());

    // Part 2
    let mut reported = HashSet::new();
    reported.insert(Point(0,0));

    let mut knots: Vec<Point> = repeat(Point(0,0)).take(KNOTS).collect();

    for d in directions.iter() {
        knots[0] = knots[0] + *d; // set head
        for _ in 0..d.n_steps() {
            for k in 1..KNOTS {
                knots[k] = follow(&knots[k], &knots[k - 1], k == KNOTS - 1, &mut reported)
                    .unwrap_or(knots[k]);
            }
        }
    }

    println!("Part 2: {}", reported.len());
}

fn follow(me: &Point, other: &Point, report: bool, reported: &mut HashSet<Point>) -> Option<Point> {
    let change = (*other - *me).singlify();
    let res = me.clone() + change;
    if &res != other {
        if report {
            reported.insert(res);
        }
        Some(res)
    } else {
        None
    }
}