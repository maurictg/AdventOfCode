use std::{fs};

use intcode::IntCodeInterpreter;

mod intcode;

#[derive(Clone)]
struct Amplifier {
    program: String
}

impl Amplifier {
    pub fn new(program: &str) -> Self {
        Self {
            program: String::from(program)
        }
    }

    pub fn run(&self, phase: i64, input: i64) -> i64 {
        let mut pc = IntCodeInterpreter::new(&self.program);
        pc.input(phase);
        pc.input(input);
        pc.run().1.unwrap()
    }
}

fn main() {
    let program = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let amp = Amplifier::new(&program);
    let mut thrust: Vec<i64> = Vec::new();

    for a in 0..5 {
        for b in (0..5).filter(|x|*x!=a) {
            for c in (0..5).filter(|x|*x!=a&&*x!=b) {
                for d in (0..5).filter(|x|*x!=a&&*x!=b&&*x!=c) {
                    for e in (0..5).filter(|x|*x!=a&&*x!=b&&*x!=c&&*x!=d) {
                        let res = amp.run(a, amp.run(b, amp.run(c, amp.run(d, amp.run(e, 0)))));
                        thrust.push(res);
                    }
                }
            }
        }
    }

    println!("Part 1: {}", thrust.iter().max().unwrap());
}