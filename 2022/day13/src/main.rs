use std::{fs, cmp::Ordering};

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntList {
    List(Vec<IntList>),
    Int(i32),
}

impl IntList {
    pub fn parse(str: &str) -> Self {
        let mut stack: Vec<IntList> = Vec::new();
        let mut chars = str.chars();
        let mut nr_chars = String::new();

        while let Some(c) = chars.next() {
            match c {
                '[' => stack.push(IntList::List(Vec::new())),
                ']' | ',' => {
                    if !nr_chars.is_empty() {
                        let nr: i32 = nr_chars.parse().unwrap();
                        nr_chars.clear();
                        match stack.last_mut().unwrap() {
                            IntList::List(l) => l.push(IntList::Int(nr)),
                            _ => panic!("Not a list!"),
                        };
                    }

                    if c == ']' && stack.len() > 1 {
                        let item = stack.pop().unwrap();
                        match stack.last_mut().unwrap() {
                            IntList::List(l) => l.push(item),
                            _ => panic!("Not a list!"),
                        };
                    }
                }
                '0'..='9' => {
                    nr_chars.push(c);
                }
                _ => unreachable!("Parsing failed, unsupported character: {}", c),
            }
        }

        assert_eq!(1, stack.len(), "Parsing error: multiple root groups found");
        assert!(nr_chars.is_empty(), "Parsing error: number characters left");
        
        stack.pop().unwrap()
    }
}

impl Ord for IntList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(o) => o,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for IntList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (IntList::List(ll), IntList::List(rl)) => {
                let mut ll = ll.clone();
                let mut rl = rl.clone();
                ll.reverse();
                rl.reverse();
                
                loop {
                    if let Some(il) = ll.pop() {
                        if let Some(ir) = rl.pop() {
                            if let Some(o) = il.partial_cmp(&ir) {
                                break Some(o);
                            }
                        } else {
                            break Some(Ordering::Less);
                        }
                    } else {
                        if rl.is_empty() {
                            break None;
                        } else {
                            break Some(Ordering::Greater);
                        }
                    }
                }
            },
            (IntList::List(_), IntList::Int(i)) => self.partial_cmp(&IntList::List(vec![IntList::Int(*i)])),
            (IntList::Int(i), IntList::List(_)) => IntList::List(vec![IntList::Int(*i)]).partial_cmp(other),
            (IntList::Int(l), IntList::Int(r)) => if l == r {
                None // continue checking
            } else {
                Some(r.cmp(l)) // in right order if left is smaller than right
            },
        }
    }
}

fn main() {
    let pairs: Vec<(IntList, IntList)> = fs::read_to_string("input.txt").unwrap()
        .split("\n\n").map(|p|p.split_once("\n").map(|(l1,l2)|(IntList::parse(l1), IntList::parse(l2))).unwrap())
        .collect();

    // Part 1
    let part1: usize = pairs.iter().enumerate().filter(|i|i.1.0.cmp(&i.1.1) == Ordering::Greater)
        .map(|i|i.0 + 1).sum();

    println!("Part 1: {}", part1);

    // Part 2
    let mut items: Vec<IntList> = pairs.iter().flat_map(|f|vec![f.0.clone(), f.1.clone()]).collect();
    let locator2 = IntList::parse("[[2]]");
    let locator6 = IntList::parse("[[6]]");

    items.push(locator2.clone());
    items.push(locator6.clone());

    items.sort_by(|a, b| b.cmp(a));
    let i2 = items.iter().position(|p|p == &locator2).unwrap() + 1;
    let i6 = items.iter().position(|p|p == &locator6).unwrap() + 1;

    println!("Part 2: {}", i2 * i6);
}