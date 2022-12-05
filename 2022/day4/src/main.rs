use std::fs;

struct Range(u8, u8);

impl Range {
    pub fn fully_contains(&self, other: &Range) -> bool {
        other.0 >= self.0 && other.1 <= self.1
    }

    pub fn does_overlap(&self, other: &Range) -> bool {
        if self.0 <= other.0 {
            self.1 >= other.0
        } else {
            other.1 >= self.0
        }
    }

    pub fn parse(str: &str) -> Self {
        let parts = str.split_once('-').unwrap();
        Self(parts.0.parse().unwrap(), parts.1.parse().unwrap())
    }
}

fn main() {
    let ranges: Vec<(Range,Range)> = fs::read_to_string("input.txt").expect("Failed to read file").lines()
        .map(|r|r.split_once(',').unwrap()).map(|(a,b)|(Range::parse(a), Range::parse(b))).collect();

    let part1 = ranges.iter().filter(|n|n.0.fully_contains(&n.1)||n.1.fully_contains(&n.0)).count();
    println!("Part 1: {}", part1);

    let part2 = ranges.iter().filter(|r|r.0.does_overlap(&r.1)).count();
    println!("Part 2: {}", part2);
}
