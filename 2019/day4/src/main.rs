use std::{fs, ops::Range};

fn main() {
    let digits: Range<i32> = fs::read_to_string("input.txt").unwrap().split_once("-")
        .and_then(|(from,to)|Some(from.parse().unwrap()..to.parse().unwrap())).unwrap();

    let valid: Vec<String> = digits.map(|d|d.to_string())
        .filter(|d|increases(d))
        .filter(|d|has_dups(d))
        .collect();

    println!("Part 1: {}", valid.len());
    println!("Part 2: {}", valid.iter().filter(|d|has_no_adj_digits(d)).count());
}

fn has_no_adj_digits(str: &str) -> bool {
    let mut parts = Vec::new();

    let mut cnt = 1;
    for i in 0..str.len() - 1 {
        let c1 = str.chars().nth(i).unwrap();
        let c2 = str.chars().nth(i + 1).unwrap();
        if c1 == c2 {
            cnt += 1;
        } else {
            parts.push(cnt);
            cnt = 1;
        }
    }
    parts.push(cnt);

    parts.iter().any(|x|*x == 2)
}

fn has_dups(str: &str) -> bool {
    for i in 0..str.len() - 1 {
        let c1 = str.chars().nth(i).unwrap();
        let c2 = str.chars().nth(i + 1).unwrap();
        if c1 == c2 {
            return true;
        }
    }

    false
}

fn increases(str: &str) -> bool {
    let mut chars = str.chars();
    let mut prev = 0;

    while let Some(c1) = chars.next() {
        let nr = c1.to_digit(10).unwrap();
        if nr < prev {
            return false;
        } else {
            prev = nr;
        }
    }

    true
}