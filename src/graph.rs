use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    phone: u64,
    following: Vec<Rc<RefCell<Node>>>,
    followers: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(datum: u64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            phone: datum,
            following: Vec::new(),
            followers: Vec::new()
        }))
    }
}

pub type Roots = HashMap<u64, Rc<RefCell<Node>>>;

pub trait RootsFeatures {
    fn with_capacity(capacity:usize) -> Self;
    fn add_edge(&mut self, x:u64, y:u64) -> &Self;
    fn lookup(&mut self, x:u64) -> Option<Vec<u64>>;
    fn rlookup(&mut self, x:u64) -> Option<Vec<u64>>;
}

impl RootsFeatures for Roots {
    fn with_capacity(capacity:usize) -> Self {
        HashMap::with_capacity(capacity)
    }

    fn add_edge(&mut self, x:u64, y:u64) -> &Self {
        let nx : &Rc<RefCell<Node>> = &self.entry(x).or_insert_with(|| Node::new(x)).clone();
        let ny : &Rc<RefCell<Node>> = &self.entry(y).or_insert_with(|| Node::new(y)).clone();
        
        {
            let mut mut_x = nx.borrow_mut();
            let mut mut_y = ny.borrow_mut();
            &mut_x.following.push(ny.clone());
            &mut_y.followers.push(nx.clone());
        }
        
        self
    }

    fn lookup(&mut self, x:u64) -> Option<Vec<u64>> {
        let x = &self.get(&x)?;
        Some (x.borrow().following.iter().map(|x| x.borrow().phone).collect::<Vec<_>>())
    }

    fn rlookup(&mut self, x:u64) -> Option<Vec<u64>> {
        let x = &self.get(&x)?;
        Some (x.borrow().followers.iter().map(|x| x.borrow().phone).collect::<Vec<_>>()) 
    }
}