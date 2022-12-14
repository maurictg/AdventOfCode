use std::fs;

use pathfinding::prelude::{bfs, Matrix};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut matrix = Matrix::from_rows(input.lines().map(|l| l.bytes().map(|b| b))).unwrap();

    let start = matrix.indices().find(|s| matrix[*s] == b'S').unwrap();
    let end = matrix.indices().find(|s| matrix[*s] == b'E').unwrap();

    matrix[start] = b'a';
    matrix[end] = b'z';

    let matrix = &matrix;

    let res = bfs(
        &start,
        |&s| {
            matrix
                .neighbours(s, false)
                .filter(move |&p| matrix[p] <= matrix[s] + 1)
        },
        |f| *f == end,
    )
    .unwrap();

    println!("Part 1: {}", res.len() - 1);

    let res = bfs(
        &end,
        |&s| {
            matrix
                .neighbours(s, false)
                .filter(move |&p| matrix[s] <= matrix[p] + 1)
        },
        |f| matrix[*f] == b'a',
    )
    .unwrap();

    println!("Part 2: {}", res.len() - 1);
}
