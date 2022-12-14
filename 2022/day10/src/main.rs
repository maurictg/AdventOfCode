use std::{fs};

struct CRT {
    pub x: i32,
    pub cycles: i32,
    pub reported: Vec<i32>,
    pub screen: String
}

enum Instruction {
    NoOp,
    AddX(i32)
}

impl Instruction {
    pub fn parse(str: &str) -> Self {
        let items: Vec<&str> = str.split_whitespace().collect();
        match *items.get(0).unwrap() {
            "noop" => Instruction::NoOp,
            "addx" => Instruction::AddX(items.get(1).unwrap().parse().unwrap()),
            _ => panic!("Invalid opcode")
        }
    }
}

impl CRT {
    pub fn new() -> Self {
        Self { x: 1, cycles: 0, reported: Vec::new(), screen: String::with_capacity(240) }
    }
    
    fn tick(&mut self) {
        let cycles = self.cycles % 40;
        let p = if cycles == self.x - 1 || cycles == self.x + 1 || cycles == self.x { '#' } else { '.' };
        self.screen.push(p);

        self.cycles += 1;
        if self.cycles == 20 || (self.cycles - 20) % 40 == 0 {
            self.reported.push(self.cycles * self.x);
        }
    }

    pub fn run(&mut self, i: Instruction) {
        self.tick();
        match i {
            Instruction::NoOp => {},
            Instruction::AddX(v) => {
                self.tick();
                self.x += v;
            },
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = fs::read_to_string("input.txt").unwrap().lines().map(|x|Instruction::parse(x)).collect();
    let mut c = CRT::new();
    for i in instructions {
        c.run(i);
    }

    println!("Part 1: {}", c.reported.iter().sum::<i32>());

    println!("Part 2: \n");
    let mut chars = c.screen.chars();
    for _ in 0..240/40 {
        for _ in 0..40 {
            print!("{}",chars.next().unwrap());
        }
        print!("\n");
    }
}
