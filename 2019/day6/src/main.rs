use std::{rc::{Rc, Weak}, cell::RefCell, fs, collections::HashMap};

#[derive(Debug)]
struct SpaceObject {
    pub name: String,
    pub children: RefCell<Vec<Rc<SpaceObject>>>,
    pub parent: RefCell<Option<Weak<SpaceObject>>>
}

fn main() {
    let input: Vec<(String,String)> = fs::read_to_string("input.txt").unwrap()
        .lines().map(|l|l.split_once(")").map(|(a,b)|(a.to_owned(), b.to_owned())).unwrap()).collect();

    let mut objects: HashMap<String, Rc<SpaceObject>> = HashMap::new();
    
    for (obj, sat) in input.iter() {
        // Add child
        let satelite = objects.entry(sat.to_owned()).or_insert(Rc::new(SpaceObject { name: sat.to_owned(), children: RefCell::new(Vec::new()), parent: RefCell::new(None) }));
        let satelite = satelite.clone();

        // Add parent
        let object = objects.entry(obj.to_owned()).or_insert(Rc::new(SpaceObject { name: obj.to_owned(), children: RefCell::new(Vec::new()), parent: RefCell::new(None) }));

        // Assign reference to parent in child
        object.children.borrow_mut().push(satelite.clone());
        *satelite.parent.borrow_mut() = Some(Rc::downgrade(&object));
    }

    // Part 1
    let com = objects.get("COM").expect("No Center of Mass found in this universe");
    println!("Part 1: {}", count_orbits(com));


    // Part 2
    let me = objects.get("YOU").expect("YOU not found");
    let santa = objects.get("SAN").expect("Santa not found");

    println!("Part 2: {}", orbital_switches_to_shared(me, santa));
}

fn orbital_switches_to_shared(a: &Rc<SpaceObject>, b: &Rc<SpaceObject>) -> i32 {
    let b_parents = get_parents(b);

    let mut up = 0;
    let mut parent = None;
    
    let mut next = a.parent.borrow().clone();
    while let Some(p) = next.clone() {
        if let Some(r) = p.upgrade() {
            up += 1;
            if b_parents.contains(&r.name) {
                parent = Some(r);
                break;
            } else {
                next = r.parent.borrow().clone();
            }
        } else {
            break;
        }
    }

    let parent = parent.expect("No common parent found");
    println!("Found shared parent {} after {} steps", &parent.name, up);

    let steps_from_b = b_parents.iter().position(|p|p == &parent.name).unwrap() as i32;
    println!("{}", steps_from_b); // 0 = target (parent of SAN)
    up - 1 + steps_from_b
}

fn get_parents(obj: &Rc<SpaceObject>) -> Vec<String> {
    let mut parents = Vec::new();
    let mut next = obj.parent.borrow().clone();
    while let Some(p) = next.clone() {
        if let Some(r) = p.upgrade() {
            parents.push(r.name.clone());
            next = r.parent.borrow().clone();
        } else {
            break;
        }
    }
    parents
}

fn count_orbits(obj: &Rc<SpaceObject>) -> i32 {
    let mut direct_orbits = 0;
    
    for c in obj.children.borrow().iter() {
        direct_orbits += 1 + count_orbits(c);
    }

    let indirect = count_indirect_orbits(obj);
    direct_orbits + if indirect > 0 { indirect - 1 } else { 0 }
}

fn count_indirect_orbits(obj: &Rc<SpaceObject>) -> i32 {
    let mut indirect_orbits = 0;

    let parent = obj.parent.borrow();
    if parent.is_some() {
        let parent = parent.as_ref().unwrap().upgrade().unwrap();
        indirect_orbits += 1 + count_indirect_orbits(&parent);
    }

    indirect_orbits
}