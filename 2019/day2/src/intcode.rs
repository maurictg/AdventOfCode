pub struct IntCodeInterpreter {
    memory: Vec<u32>,
    pos: usize
}

pub enum Action {
    Sum(usize, usize, usize),
    Multiply(usize, usize, usize),
    Halt
}

impl IntCodeInterpreter {
    pub fn new(mem: &str) -> Self {
        Self {
            memory: mem.split(',').map(|c|c.parse().unwrap()).collect(),
            pos: 0
        }
    }

    pub fn set(&mut self, noun: u32, verb: u32) {
        *self.memory.get_mut(1).unwrap() = noun;
        *self.memory.get_mut(2).unwrap() = verb;
    }

    pub fn run(mut self) -> u32 {
        while self.pos < self.memory.len() {
            let mem = &self.memory[self.pos..];
            match self.execute(match mem[0] {
                1 => Action::Sum(mem[1] as usize, mem[2] as usize, mem[3] as usize),
                2 => Action::Multiply(mem[1] as usize, mem[2] as usize, mem[3] as usize),
                99 => Action::Halt,
                _ => panic!("!")
            }) {
                Some(p) => self.pos += p,
                None => break,
            }
        }

        *self.memory.get(0).unwrap()
    }

    fn execute(&mut self, action: Action) -> Option<usize> {
        match action {
            Action::Sum(x, y, z) => {
                let res = self.memory.get(x).unwrap() + self.memory.get(y).unwrap();
                *self.memory.get_mut(z).unwrap() = res;
            },
            Action::Multiply(x, y, z) => {
                let res = self.memory.get(x).unwrap() * self.memory.get(y).unwrap();
                *self.memory.get_mut(z).unwrap() = res;
            },
            Action::Halt => { return None; }
        }

        Some(4)
    }
}