use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    incoming: Vec<Rc<RefCell<Node<T>>>>,
    outgoing: Vec<Rc<RefCell<Node<T>>>>,
}

type RefNode<T> = Rc<RefCell<Node<T>>>;

impl<T: Eq + Hash + Copy> Node<T> {
    fn new(value: T) -> RefNode<T> {
        Rc::new(RefCell::new(Node {
            value: value,
            incoming: Vec::with_capacity(55),
            outgoing: Vec::with_capacity(55)
        }))
    }
    
    fn traverse_incoming(&self, seen: &mut HashSet<T>)
    {
        if seen.contains(&self.value) {
            return;
        }
        seen.insert(self.value);
        for n in &self.incoming {
            n.borrow().traverse_incoming(seen);
        }
    }
}

pub type GraphMap<T> = HashMap<T, RefNode<T>>;

pub trait GraphMapFeatures<T> {
    fn with_capacity(capacity:usize) -> Self;
    fn add_node(&mut self, x:T) -> RefNode<T>;
    fn add_edge(&mut self, x:&RefNode<T>, y:&RefNode<T>) -> &Self;
    fn lookup(&mut self, x:T) -> Option<Vec<T>>;
    fn rlookup(&mut self, x:T) -> Option<Vec<T>>;
    fn suggest(&self, x:T) -> Option<Vec<T>>;
}

impl<T:Eq + Hash + Copy> GraphMapFeatures<T> for GraphMap<T> {
    fn with_capacity(capacity:usize) -> Self {
        HashMap::with_capacity(capacity)
    }

    fn add_node(&mut self, x:T) -> RefNode<T> {
        self.entry(x).or_insert_with(|| Node::new(x)).clone()
    }

    fn add_edge(&mut self, x:&RefNode<T>, y:&RefNode<T>) -> &Self {
        
        {
            let mut mut_x = x.borrow_mut();
            &mut_x.incoming.push(y.clone());
        }
        {
            let mut mut_y = y.borrow_mut();
            &mut_y.outgoing.push(x.clone());
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

    fn suggest(&self, x:T) -> Option<Vec<T>> {
        let node = &self.get(&x)?;
        let mut top : HashSet<T> = HashSet::new();
        
        let alreadyfollowed = &node.borrow().incoming.iter().map(|x| x.borrow().value).collect::<HashSet<_>>();

        let myself = *&node.borrow().value;
        &node.borrow().traverse_incoming(&mut top);
        let result = top.difference(alreadyfollowed).into_iter().map (|x| *x).filter(|x| *x != myself).take(10).collect::<Vec<_>>();
        Some (result)
    }
}