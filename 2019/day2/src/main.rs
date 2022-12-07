use std::{fs, process::exit};

use intcode::IntCodeInterpreter;

mod intcode;

const EXPECTED: u32 = 19690720;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let mut computer = IntCodeInterpreter::new(&input);
    computer.set(12, 02);

    let res = computer.run();
    println!("Part 1: {}", res);

    // Part 2
    for noun in 0u32..=99 {
        for verb in 0u32..=99 {
            let mut computer = IntCodeInterpreter::new(&input);
            computer.set(noun, verb);
            if computer.run() == EXPECTED {
                println!("Part 2: {:02}{:02}", noun, verb);
                exit(0);
            }
        }
    }
}
