use std::{fs, collections::{VecDeque, HashMap}};

use itertools::Itertools;
use regex::{Regex};

struct Monkey {
    pub number: i32,
    operation: Box<dyn Fn(usize) -> usize>,
    items: VecDeque<usize>,
    test: usize,
    if_true: i32,
    if_false: i32,
    pub items_inspected: usize
}

impl Monkey {
    pub fn new(str: &str) -> Self {
        let regex = Regex::new(r"^Monkey\s(\d+):\n.*items:(.*)\n\s\sOperation:\snew\s=\s(.*)\n.*by\s(\d+)\n\s+.*monkey\s(\d+)\n.*monkey\s(\d+)$").unwrap();
        let matches = regex.captures(str).expect("Not a monkey!");
        
        let get_int = |i| {
            matches.get(i).unwrap().as_str().parse().unwrap()
        };

        let operation = matches.get(3).unwrap().as_str();
        let operation: Box<dyn Fn(usize) -> usize> = if operation.starts_with("old *") {
            if operation.ends_with("old") {
                Box::new(|f|f * f)
            } else {
                let nr: usize = operation.split_whitespace().last().unwrap().parse().unwrap();
                Box::new(move|f|f * nr)
            }
        } else if operation.starts_with("old +") {
            if operation.ends_with("old") {
                Box::new(|f|f + f)
            } else {
                let nr: usize = operation.split_whitespace().last().unwrap().parse().unwrap();
                Box::new(move|f|f + nr)
            }
        } else {
            unreachable!("Operation not supported: {}", operation);
        };
        
        Self {
            number: get_int(1),
            items: matches.get(2).unwrap().as_str().replace(" ", "").split(',').map(|s|s.parse().unwrap()).collect(),
            test: get_int(4) as usize,
            if_true: get_int(5),
            if_false: get_int(6),
            items_inspected: 0,
            operation
        }
    }

    pub fn play_round(&mut self, modulo: Option<usize>) -> Vec<(i32, usize)> {
        let mut actions = Vec::new();
        while let Some(item) = self.items.pop_front() {
            let mut worry_level = (*self.operation)(item);

            if modulo.is_none() {
                // Monkey gets bored. If no damage, worry / 3 rounded down
                worry_level /= 3;
            }

            let next_monkey = if worry_level % self.test == 0 {
                self.if_true
            } else {
                self.if_false
            };

            actions.push((next_monkey, modulo.map(|m|worry_level % m).unwrap_or(worry_level)));
            self.items_inspected += 1;
        }

        actions
    }

    pub fn pass(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

fn main() {
    // Part 1
    let mut monkeys: HashMap<i32, Monkey> = fs::read_to_string("input.txt").unwrap().split("\n\n").map(|m|Monkey::new(m)).map(|m|(m.number, m)).collect();

    for _ in 0..20 {
        for i in 0..(monkeys.len() as i32) {
            let monkey = monkeys.get_mut(&i).unwrap();
            let res = monkey.play_round(None);
            for (next, item) in res {
                let next_monkey = monkeys.get_mut(&next).unwrap();
                next_monkey.pass(item);
            }
        }
    }

    let monkey_business: usize = monkeys.values().map(|i|i.items_inspected)
        .sorted_by(|a,b|b.cmp(a)).take(2).product();
    
    println!("Part 1: {}", monkey_business);

    // Part 2: modulo trick
    let mut monkeys: HashMap<i32, Monkey> = fs::read_to_string("input.txt").unwrap().split("\n\n").map(|m|Monkey::new(m)).map(|m|(m.number, m)).collect();
    let modulo: usize = monkeys.values().map(|x|x.test).product();    

    for _ in 0..10000 {
        for i in 0..(monkeys.len() as i32) {
            let monkey = monkeys.get_mut(&i).unwrap();
            let res = monkey.play_round(Some(modulo));
            for (next, item) in res {
                let next_monkey = monkeys.get_mut(&next).unwrap();
                next_monkey.pass(item);
            }
        }
    }

    let monkey_business: usize = monkeys.values().map(|i|i.items_inspected)
        .sorted_by(|a,b|b.cmp(a)).take(2).product();
    
    println!("Part 2: {}", monkey_business);
}
