use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6
}

impl RockPaperScissors {
    pub fn new(letter: &str) -> Self {
        match letter {
            "A" | "X" => RockPaperScissors::Rock,
            "B" | "Y" => RockPaperScissors::Paper,
            "C" | "Z" => RockPaperScissors::Scissors,
            _ => panic!("Unknown code: {}", letter)
        }
    }

    pub fn beats(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper
        }
    }

    pub fn defeats(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Scissors => RockPaperScissors::Rock,
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors
        }
    }

    pub fn play(&self, other: &Self) -> Outcome {
        if self == other {
            Outcome::Draw
        } else if other == &self.beats() {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }
}

impl Outcome {
    pub fn new(letter: &str) -> Self {
        match letter {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknown code: {}", letter)
        }
    }

    pub fn desired_outcome(&self, opponent: &RockPaperScissors) -> RockPaperScissors {
        match self {
            Outcome::Lose => opponent.beats(),
            Outcome::Draw => *opponent,
            Outcome::Win => opponent.defeats(),
        }
    }
}

fn main() {
    // can also use include_str! macro
    let guide: Vec<(String, String)> = fs::read_to_string("input.txt").expect("Failed to read input file")
        .lines().map(|l|l.split_once(" ").unwrap()).map(|(a,b)|(a.to_owned(), b.to_owned())).collect();

    // Part 1
    let part1: i32 = guide.iter().map(|(other, me)|(RockPaperScissors::new(other), RockPaperScissors::new(me)))
        .map(|(other, me)| me.play(&other) as i32 + me as i32).sum();
    
    println!("Part 1: {}", part1);

    // Part 2
    let part2: i32 = guide.iter().map(|(other, desired)|(RockPaperScissors::new(other), Outcome::new(desired)))
        .map(|(other, desired)|(other, desired.desired_outcome(&other)))
        .map(|(other, me)| me.play(&other) as i32 + me as i32).sum();

    println!("Part 2: {}", part2);
}