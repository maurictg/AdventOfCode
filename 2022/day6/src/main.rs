use std::fs;

use itertools::Itertools;

fn main() {
    let signal = fs::read_to_string("input.txt").expect("Failed to read input file");
    
    println!("Part 1: {}", find_marker(&signal, 4));
    println!("Part 2: {}", find_marker(&signal, 14));
}

fn find_marker(str: &String, len: usize) -> usize {
    for i in 0..str.len() - len {
        let chars: Vec<char> = str[i..i+len].chars().collect();
        if chars.len() == chars.iter().unique().count() {
            return i + len;
        }
    }

    panic!("Not found!");
}