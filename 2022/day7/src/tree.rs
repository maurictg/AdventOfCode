use std::{rc::Rc, cell::{RefCell}, collections::HashMap};

pub struct NodeIter<'a, T>(&'a Node<T>, usize, Rc<RefCell<Arena<T>>>);

#[derive(Debug)]
pub struct Arena<T> {
    pub counter: u32,
    pub nodes: HashMap<u32, Rc<RefCell<Node<T>>>>
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub id: Option<u32>,
    pub parent: Option<u32>,
    children: Vec<u32>
}

impl<T> Node<T> {
    pub fn root(arena: Rc<RefCell<Arena<T>>>, val: T) -> Rc<RefCell<Self>> {
        let root_node = Node { value: val, parent: None, id: None, children: Vec::new() };
        let id = arena.borrow_mut().add(root_node);
        arena.borrow().get(id)
    }

    pub fn add_child(&mut self, val: T, arena: Rc<RefCell<Arena<T>>>) {
        let node = Node { value: val, parent: self.id, id: None, children: Vec::new()};
        let id = arena.borrow_mut().add(node);
        self.children.push(id);
    }

    pub fn iter(&self, arena: Rc<RefCell<Arena<T>>>) -> NodeIter<T> {
        NodeIter(self, 0, arena)
    }

    /* pub fn visit(&self, arena: Rc<RefCell<Arena<T>>>, fun: Box<fn(&T)>) {
        for child in self.iter(arena.clone()) {
            child.borrow().visit(arena.clone(), fun.clone());
        }
        fun(&self.value);
    } */
}

impl<T> Arena<T> {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            counter: 0,
            nodes: HashMap::new()
        }))
    }

    fn add(&mut self, mut node: Node<T>) -> u32 {
        self.counter += 1;
        node.id = Some(self.counter);
        self.nodes.insert(self.counter, Rc::new(RefCell::new(node)));
        self.counter
    }

    pub fn get(&self, id: u32) -> Rc<RefCell<Node<T>>> {
        self.nodes.get(&id).unwrap().clone()
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < self.0.children.len() {
            let id = self.0.children.get(self.1).unwrap();
            self.1 += 1;
            Some(self.2.borrow().get(*id))
        } else {
            None
        }
    }
}