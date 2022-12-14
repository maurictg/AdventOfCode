use std::fs;

use direction::Direction;
use point::Point;

mod point;
mod direction;

#[derive(Debug, Clone)]
struct Path(Vec<Point>);

fn main() {
    let paths: Vec<Path> = fs::read_to_string("input.txt").unwrap().lines()
        .map(|l|Path(l.split(" -> ").map(|p|Point::parse(p)).collect())).collect();

    let mut grid = [[false; 1000]; 1000];

    // Part 1
    map_rocks(paths.clone(), &mut grid);

    let landed = simulate(grid);
    println!("Part 1: {}", landed);

    // Part 2
    reset_grid(&mut grid);

    let y_max = map_rocks(paths, &mut grid);
    let assumed_floor = (y_max + 2) as usize;
    for i in 0..grid[0].len() {
        grid[assumed_floor][i] = true;
    }

    let landed = simulate(grid);
    println!("Part 2: {}", landed);
}

fn simulate(mut grid: [[bool; 1000]; 1000]) -> i32 {
    let sand_origin = Point(500, 0);
    let mut landed = 0;
    
    'main: loop {
        let mut item = sand_origin;
        loop {
            if let Some(next) = can_move(item, &grid) {
                item = next;
                if below_is_infinity(item, &grid) {
                    break 'main;
                }
            } else {
                set(item, &mut grid);
                landed += 1;
                if item == sand_origin {
                    break 'main;
                }

                break;
            }
        }
    }

    landed
}

fn map_rocks(rocks: Vec<Path>, grid: &mut [[bool; 1000]; 1000]) -> i32 {
    let mut y_max = 0;
    for p in rocks.iter() {
        for i in 0..p.0.len() - 1 {
            let from = &p.0[i];
            let to = &p.0[i + 1];
            for p in from.points_between(to) {
                set(p, grid);
                if p.1 > y_max {
                    y_max = p.1;
                }
            }
        }
    }

    y_max
}

fn below_is_infinity(p: Point, grid: &[[bool; 1000]; 1000]) -> bool {
    let mut y = p.1 as usize;
    while y < 1000 {
        if grid[y][p.0 as usize] {
            return false;
        }
        y += 1;
    }

    true
}

fn get(p: Point, grid: &[[bool; 1000]; 1000]) -> bool {
    grid[p.1 as usize][p.0 as usize]
}

fn set(p: Point, grid: &mut [[bool; 1000]; 1000]) {
    grid[p.1 as usize][p.0 as usize] = true;
}

fn reset_grid(grid: &mut [[bool; 1000]; 1000]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            grid[y][x] = false;
        }
    }
}

fn can_move(item: Point, grid: &[[bool; 1000]; 1000]) -> Option<Point> {
    let mut next = item + Direction::Down(-1);
    if get(next, grid) {
        next = next + Direction::Left(1);
        if get(next, grid) {
            next = item + Direction::Down(-1) + Direction::Right(1);
            if get(next, grid) {
                return None;
            }
        }
    }

    Some(next)
}