use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let (height, width) = (grid.len(), grid.get(0).unwrap().len());

    let mut visible = 0;
    let mut scenic_scores = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                // Part 1, if on edge it is visible
                visible += 1;
            } else {
                let l = view_distance(&grid, x, y, Direction::Left);
                let r = view_distance(&grid, x, y, Direction::Right);
                let u = view_distance(&grid, x, y, Direction::Up);
                let d = view_distance(&grid, x, y, Direction::Down);

                // Part 1, if visible from any edge
                if (l.0 == x && l.1) || (r.0 == width - x - 1 && r.1)
                    || (u.0 == y && u.1) || (d.0 == height - y - 1 && d.1) {
                        println!("Visible: {} {}", x,y);
                        visible += 1;
                }

                // Part 2, scenic scores
                scenic_scores.push(l.0 * r.0 * u.0 * d.0);
            }
        }
    }

    println!("Part 1: {}", visible);
    println!("Part 2: {}", scenic_scores.iter().max().unwrap());
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn view_distance(grid: &Vec<Vec<u8>>, x: usize, y: usize, direction: Direction) -> (usize, bool) {
    let self_height = grid[y][x];
    let (height, width) = (grid.len(), grid.get(0).unwrap().len());

    match direction {
        Direction::Left => {
            for lx in (0..x).rev() {
                let h = grid[y][lx];
                if h >= self_height {
                    return (x - lx, !h == self_height);
                }
            }

            (x, self_height >= grid[y][0])
        }
        Direction::Right => {
            for lx in (x + 1)..width {
                let h = grid[y][lx];
                if h >= self_height {
                    return (lx - x, !h == self_height);
                }
            }

            (width - x - 1, self_height >= grid[y][width - 1])
        }
        Direction::Up => {
            for ly in (0..y).rev() {
                let h = grid[ly][x];
                if h >= self_height {
                    return (y - ly, !h == self_height);
                }
            }

            (y, self_height >= grid[0][x])
        }
        Direction::Down => {
            for ly in (y + 1)..height {
                let h = grid[ly][x];
                if h >= self_height {
                    return (ly - y, !h == self_height);
                }
            }

            (height - y - 1, self_height >= grid[height - 1][x])
        }
    }
}
