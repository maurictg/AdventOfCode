use std::fs;

#[derive(Debug)]
struct Point(i16,i16);

#[derive(Debug)]
struct Line(Point, Point);

impl Point {
    fn new(str: &str) -> Self {
        let mut coords = str.split(',');
        Point(coords.next().unwrap().parse().unwrap(), coords.next().unwrap().parse().unwrap())
    }
}

fn main() {
    let lines: Vec<Line> = fs::read_to_string("input.txt").expect("Failed to read input file").lines()
        .map(|l|l.split("->").map(|ps|Point::new(ps.trim()))).map(|mut ps|Line(ps.next().unwrap(),ps.next().unwrap())).collect();

    let map = [[0u8; 1000]; 1000];
}
