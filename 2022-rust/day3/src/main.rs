use std::fs;

use itertools::Itertools;

trait RucksackItem {
    fn priority(&self) -> u32;
}

impl RucksackItem for char {
    fn priority(&self) -> u32 {
        match self {
            'a'..='z' => *self as u32 - 96,
            'A'..='Z' => *self as u32 - 38,
            _ => panic!("Invalid item: {}", self)
        }
    }
}

fn main() {
    let rucksacks: Vec<String> = fs::read_to_string("input.txt").expect("Failed to read input file").lines().map(|s|s.to_owned()).collect();
    
    let mut vals = 0;

    for r in rucksacks.iter().map(|l|(l[..l.len()/2].to_owned(), l[l.len()/2..].to_owned())) {
        for c in r.0.chars().unique() {
            if r.1.contains(c) {
                vals += c.priority();
            }
        }
    }
    
    println!("Part 1: {}", vals);

    let mut vals = 0;
    for group in rucksacks.chunks(3) {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                vals += c.priority();
                break;
            }
        }
    }

    println!("Part 2: {}", vals);
}
