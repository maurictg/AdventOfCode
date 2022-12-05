use std::{fs, iter::repeat};

struct Ship {
    pub stacks: Vec<Vec<char>>
}

impl Ship {
    pub fn new(str: &str) -> Self {
        let stack_cnt = str.lines().last().unwrap().len() / 4 + 1;
        let mut ship = Self { stacks: repeat(Vec::new()).take(stack_cnt).collect() };
        
        for s in str.lines() {
            let mut stack_nr = 1;
            for item in s.chars().collect::<Vec<char>>().chunks(4).map(|c|String::from_iter(c).trim().to_owned()) {
                if item.len() == 3 {
                    ship.stacks.get_mut(stack_nr - 1).unwrap().insert(0, item.chars().nth(1).unwrap());
                }
                stack_nr += 1;
            }
        }

        ship
    }
    
    pub fn move_containers_cm9000(&mut self, n: usize, from: usize, to: usize) {
        for _ in 1..=n {
            let from = self.stacks.get_mut(from - 1).expect("Could not get 'from' stack");
            let container = from.pop().expect("From stack is empty!");
            let to = self.stacks.get_mut(to - 1).expect("Could not get 'to' stack");
            to.push(container);
        }
    }

    pub fn move_containers_cm9001(&mut self, n: usize, from: usize, to: usize) {
        let from = self.stacks.get_mut(from - 1).expect("Could not get 'from' stack");
        let tail = from.split_off(from.len() - n);
        let to = self.stacks.get_mut(to - 1).expect("Could not get 'to' stack");
        for t in tail.iter() {
            to.push(*t);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let (ship_str, instructions) = input.split_once("\n\n").unwrap();

    let mut ship = Ship::new(ship_str);
    let mut ship2 = Ship::new(ship_str);

    for instruction in instructions.lines() {
        let mut vals = instruction.split_whitespace().filter(|n|n.chars().all(|c|c.is_numeric())).map(|i|i.parse::<usize>().unwrap());
        let (n,from,to) = (vals.next().unwrap(), vals.next().unwrap(), vals.next().unwrap());
        ship.move_containers_cm9000(n, from, to);
        ship2.move_containers_cm9001(n, from, to);
    }

    let top_row = String::from_iter(ship.stacks.iter().map(|s|s.last().unwrap()));
    println!("Part 1: {}", top_row); //QPJPLMNNR

    let top_row = String::from_iter(ship2.stacks.iter().map(|s|s.last().unwrap()));
    println!("Part 2: {}", top_row); //BQDNWJPVJ
}
