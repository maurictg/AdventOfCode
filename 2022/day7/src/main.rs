mod tree;

#[derive(Debug)]
enum Type {
    File(String, u32),
    Folder(String, RefCell<Option<u32>>),
}

use std::{fs, rc::Rc, cell::RefCell};
use tree::{Arena, Node};

use crate::Type::{File, Folder};

fn main() {
    let instructions: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect();

    let arena = Arena::new();
    let root = Node::root(arena.clone(), Folder(String::from("/"), RefCell::new(None)));
    let mut cursor: u32 = root.borrow().id.unwrap();

    // First instruction is always cd /
    for i in instructions.iter().skip(1) {
        if i.starts_with('$') {
            // command
            //println!("Executing command: {}", i);
            let command: Vec<&str> = i.split_whitespace().skip(1).collect();
            match *command.get(0).unwrap() {
                "ls" => continue,
                "cd" => {
                    let node = arena.clone().borrow_mut().get(cursor);
                    let name = String::from(*command.get(1).unwrap());

                    if &name == ".." {
                        cursor = node.borrow().parent.unwrap();
                    } else {
                        for t in node.borrow().iter(arena.clone()) {
                            if let Folder(f, _) = &t.borrow().value {
                                if f == &name {
                                    cursor = t.borrow().id.unwrap();
                                }
                            }
                        }
                    }
                }
                _ => panic!("Unknown command"),
            }
        } else {
            // content
            let (size_or_type, name) = i.split_once(' ').unwrap();
            let node = arena.clone().borrow_mut().get(cursor);

            if size_or_type == "dir" {
                //println!("Creating folder {} in {}", name, cursor);
                node.borrow_mut()
                    .add_child(Folder(String::from(name), RefCell::new(None)), arena.clone());
            } else {
                //println!("Creating file {} in {}", name, cursor);
                node.borrow_mut().add_child(
                    File(String::from(name), size_or_type.parse().unwrap()),
                    arena.clone(),
                );
            }
        }
    }


    fn calculate_sizes(node: Rc<RefCell<Node<Type>>>, arena: Rc<RefCell<Arena<Type>>>) -> u32 {
        let mut size = 0;
        for item in node.borrow().iter(arena.clone()) {
            match &item.borrow().value {
                File(_, s) => size += *s,
                Folder(_, s) => {
                    let folder_size = calculate_sizes(item.clone(), arena.clone());
                    size += folder_size;
                    let _ = s.borrow_mut().insert(folder_size);
                }
            }
        }

        size
    }

    let space_used = calculate_sizes(root.clone(), arena.clone());
    println!("total size: {}", space_used);

    let total_disk_space = 70000000;
    let update_size = 30000000;
    let left = total_disk_space - space_used;
    let needed = (left as i32 - update_size as i32).abs() as u32;

    let mut best_folder = u32::MAX;

    // Part 1, 2
    let mut total = 0;
    for node in arena.borrow().nodes.values() {
        match &node.borrow().value {
            File(_, _) => {},
            Folder(_, s) => {
                total += match *s.borrow() {
                    Some(i) => {
                        if i >= needed && i < best_folder {
                            best_folder = i;
                        }

                        if i <= 100000 { i } else { 0 }
                    },
                    _ => 0,
                };
            },
        }
    }

    println!("Part 1: {}", total);
    println!("Part 2: {}", best_folder);
}
