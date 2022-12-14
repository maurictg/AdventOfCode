use std::{fs, collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    pub fn edges(&self) -> Vec<Point> {
        let mut e = Vec::with_capacity(4);
        e.push(Point(self.0 + 1, self.1));
        e.push(Point(self.0, self.1 + 1));
        if self.0 > 0 {
            e.push(Point(self.0 - 1, self.1));
        }
        if self.1 > 0 {
            e.push(Point(self.0, self.1 - 1));
        }
        e
    }
}

fn main() {
    let mut matrix: Vec<Vec<u8>> = fs::read_to_string("input.txt").unwrap().lines()
        .map(|l|l.bytes().collect()).collect();

    let mut start = Point(0,0);
    let mut end = Point(0,0);

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] == b'S' {
                start = Point(x, y);
                matrix[y][x] = b'a';
            }
            if matrix[y][x] == b'E' {
                end = Point(x, y);
                matrix[y][x] = b'z';
            }
        }
    }

    let part1 = bfs(start, &matrix, |p|*p==end, |next, p|
        matrix[p.1][p.0] <= matrix[next.1][next.0] + 1
    );

    let part2 = bfs(end, &matrix, |p|matrix[p.1][p.0] == b'a', |next, p|
        matrix[next.1][next.0] <= matrix[p.1][p.0] + 1
    );

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn bfs<F1, F2>(start: Point, matrix: &Vec<Vec<u8>>, goal: F1, filter: F2) -> usize
    where F1: Fn(&Point) -> bool, F2: Fn(&Point, &Point) -> bool
{
    let width = matrix[0].len();
    let height = matrix.len();

    let mut queue = Vec::new();
    let mut visited: HashMap<Point, Point> = HashMap::new();

    queue.insert(0, start);

    let mut target = None;

    while let Some(next) = queue.pop() {
        if goal(&next) {
            target = Some(next);
            break;
        }

        for edge in next.edges().iter()
            .filter(|p|p.0 < width && p.1 < height)
            .filter(|p|filter(&next, *p)) {
                if visited.get(edge).is_none() {
                    visited.insert(*edge, next);
                    queue.insert(0, *edge);
                }
            }
    }

    let mut path = Vec::new();
    let mut p = target.unwrap();
    path.push(p);

    while p != start {
        p = *visited.get(&p).unwrap();
        path.push(p);
    }

    path.reverse();
    path.len() - 1
}