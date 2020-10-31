use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    incoming: Vec<Rc<RefCell<Node<T>>>>,
    outgoing: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            incoming: Vec::new(),
            outgoing: Vec::new()
        }))
    }
}

pub type GraphMap<T> = HashMap<T, Rc<RefCell<Node<T>>>>;

pub trait GraphMapFeatures<T> {
    fn with_capacity(capacity:usize) -> Self;
    fn add_edge(&mut self, x:T, y:T) -> &Self;
    fn lookup(&mut self, x:T) -> Option<Vec<T>>;
    fn rlookup(&mut self, x:T) -> Option<Vec<T>>;
}

impl<T:Eq + Hash + Copy> GraphMapFeatures<T> for GraphMap<T> {
    fn with_capacity(capacity:usize) -> Self {
        HashMap::with_capacity(capacity)
    }

    fn add_edge(&mut self, x:T, y:T) -> &Self {
        let nx : &Rc<RefCell<Node<T>>> = &self.entry(x).or_insert_with(|| Node::new(x)).clone();
        let ny : &Rc<RefCell<Node<T>>> = &self.entry(y).or_insert_with(|| Node::new(y)).clone();
        
        {
            let mut mut_x = nx.borrow_mut();
            let mut mut_y = ny.borrow_mut();
            &mut_x.incoming.push(ny.clone());
            &mut_y.outgoing.push(nx.clone());
        }
        
        self
    }

    fn lookup(&mut self, x:T) -> Option<Vec<T>> {
        let x = &self.get(&x)?;
        Some (x.borrow().incoming.iter().map(|x| x.borrow().value).collect::<Vec<_>>())
    }

    fn rlookup(&mut self, x:T) -> Option<Vec<T>> {
        let x = &self.get(&x)?;
        Some (x.borrow().outgoing.iter().map(|x| x.borrow().value).collect::<Vec<_>>()) 
    }
}