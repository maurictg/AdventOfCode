pub struct IntCodeInterpreter {
    memory: Vec<i64>,
    pos: usize,
    input: Option<i64>,
    output: Option<i64>,
    debug_mode: bool
}

#[derive(Debug)]
pub enum Parameter {
    Position(usize),
    Immediate(i64)
}

#[derive(Debug)]
pub enum Action {
    Sum(Parameter, Parameter, usize),
    Multiply(Parameter, Parameter, usize),
    Save(usize),
    Load(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equals(Parameter, Parameter, usize),
    Halt
}

impl IntCodeInterpreter {
    pub fn new(mem: &str) -> Self {
        Self {
            memory: mem.split(',').map(|c|c.parse().unwrap()).collect(),
            pos: 0,
            input: None,
            output: None,
            debug_mode: false
        }
    }

    pub fn set(&mut self, noun: i64, verb: i64) {
        *self.memory.get_mut(1).unwrap() = noun;
        *self.memory.get_mut(2).unwrap() = verb;
    }

    pub fn enable_debug(&mut self) {
        self.debug_mode = true;
    }

    fn get_mem(&mut self, param: Parameter) -> i64 {
        match param {
            Parameter::Position(p) => {
                *self.memory.get(p).unwrap()
            },
            Parameter::Immediate(val) => {
                val
            },
        }
    }

    pub fn run(mut self, input: Option<i64>) -> (i64, Option<i64>) {
        self.input = input;
        while self.pos < self.memory.len() {
            let mem = &self.memory[self.pos..];

            let param_as = |immediate: bool, idx: usize| -> Parameter {
                let data = mem[idx];
                if immediate {
                    Parameter::Immediate(data)
                } else {
                    Parameter::Position(data as usize)
                }
            };

            // Get OPCODE
            let opcode = mem[0];
            let settings = format!("{:05}", opcode);
            let opcode: i64 = settings[settings.len()-2..].parse().unwrap();
            let a = settings.chars().nth(2).unwrap() == '1';
            let b = settings.chars().nth(1).unwrap() == '1';
            // let c = settings.chars().nth(0).unwrap() == '1';

            match self.execute(match opcode {
                1 => Action::Sum(param_as(a, 1), param_as(b, 2), mem[3] as usize),
                2 => Action::Multiply(param_as(a, 1), param_as(b, 2), mem[3] as usize),
                3 => Action::Save(mem[1] as usize),
                4 => Action::Load(param_as(a, 1)),
                5 => Action::JumpIfTrue(param_as(a, 1), param_as(b, 2)),
                6 => Action::JumpIfFalse(param_as(a, 1), param_as(b, 2)),
                7 => Action::LessThan(param_as(a, 1), param_as(b, 2), mem[3] as usize),
                8 => Action::Equals(param_as(a, 1), param_as(b, 2), mem[3] as usize),
                99 => Action::Halt,
                _ => panic!("Unknown action: OP {} ({})", opcode, mem[0])
            }) {
                Some(p) => self.pos += p,
                None => break,
            }
        }

        if self.debug_mode {
            println!("Dump: {:?}", self.memory);
        }

        (*self.memory.get(0).unwrap(), self.output)
    }

    fn execute(&mut self, action: Action) -> Option<usize> {
       
        if self.debug_mode {
            println!("{:?}", action);
        }

        match action {
            Action::Sum(x, y, z) => {
                let res = self.get_mem(x) + self.get_mem(y);
                *self.memory.get_mut(z).unwrap() = res;
            },
            Action::Multiply(x, y, z) => {
                let res = self.get_mem(x) * self.get_mem(y);
                *self.memory.get_mut(z).unwrap() = res;
            },
            Action::Save(i) => {
                *self.memory.get_mut(i).unwrap() = self.input.expect("No input!");
                return Some(2);
            },
            Action::Load(i) => {
                let value = self.get_mem(i);
                self.output = Some(value);
                return Some(2);
            },
            Action::JumpIfTrue(p, i) => {
                if self.get_mem(p) != 0 {
                    let pointer = self.get_mem(i) as usize;
                    self.pos = pointer;
                    return Some(0);
                } else {
                    return Some(3);
                }
            },
            Action::JumpIfFalse(p, i) => {
                if self.get_mem(p) == 0 {
                    let pointer = self.get_mem(i) as usize;
                    self.pos = pointer;
                    return Some(0);
                } else {
                    return Some(3);
                }
            },
            Action::LessThan(a, b, c) => {
                *self.memory.get_mut(c).unwrap() = if self.get_mem(a) < self.get_mem(b) {
                    1
                } else {
                    0
                }
            },
            Action::Equals(a, b, c) => {
                *self.memory.get_mut(c).unwrap() = if self.get_mem(a) == self.get_mem(b) {
                    1
                } else {
                    0
                }
            },
            Action::Halt => { return None; },
        }

        Some(4)
    }
}