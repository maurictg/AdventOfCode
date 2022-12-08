use std::{rc::Rc, fs};

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
        let pc = IntCodeInterpreter::new(&self.program);
        // pc.enable_debug();
        pc.run(Some(&[phase, input])).1.unwrap()
    }
}

fn main() {
    let program = fs::read_to_string("input.txt").unwrap();
    let amp = Amplifier::new(&program);
    println!("{}", amp.run(0, 0));
}