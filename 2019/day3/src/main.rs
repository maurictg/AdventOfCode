use std::fs;

use lines::{Wire, Point, Line};

mod lines;
mod line_algorithms;

impl Wire {
    pub fn new(instructions: &str, start_point: Point) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let instructions: Vec<(char, i32)> = instructions
            .split(',')
            .map(|c| {
                (
                    c.chars().next().unwrap(),
                    String::from_iter(c.chars().skip(1)).parse().unwrap(),
                )
            })
            .collect();

        let mut location = start_point; // central port

        for (dir, n) in instructions {
            let new_point = match dir {
                'R' => Point(location.0 + n, location.1),
                'L' => Point(location.0 - n, location.1),
                'U' => Point(location.0, location.1 + n),
                'D' => Point(location.0, location.1 - n),
                _ => panic!("Invalid direction: {}", dir),
            };

            lines.push(Line(location.clone(), new_point.clone()));
            location = new_point;
        }

        Self(lines)
    }
}

fn main() {
    let central_port = Point(1, 1);
    let input = fs::read_to_string("input.txt").unwrap();
    let (i1, i2) = input.split_once("\n").unwrap();

    let wire1 = Wire::new(i1, central_port);
    let wire2 = Wire::new(i2, central_port);

    // Part 1
    let mut closest = i32::MAX;

    for l1 in wire1.0.iter() {
        for l2 in wire2.0.iter() {
            if let Some(x) = l1.intersect(l2) {
                let distance = x.0.distance(&central_port);
                if distance > 0 && distance < closest {
                    closest = distance;
                }
            }
        }
    }

    println!("Part 1: {}", closest);

    // PART 2 does NOT work using line algorithms. Try a grid!
}