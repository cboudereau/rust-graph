use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
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

    fn traverse<F>(&self, f: &F, seen: &mut HashSet<u64>)
        where F: Fn(u64)
    {
        if seen.contains(&self.phone) {
            return;
        }
        f(self.phone);
        seen.insert(self.phone);
        for n in &self.following {
            n.borrow().traverse(f, seen);
        }
    }

    fn first(&self) -> Rc<RefCell<Node>> {
        self.following[0].clone()
    }
}

fn foo(node: &Node) {
    println!("foo: {}", node.phone);
}

type Roots = HashMap<u64, Rc<RefCell<Node>>>;

trait RootsFeatures {
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

#[test]
fn from_sample_lookup_and_rlookup() {
    let mut roots = Roots::with_capacity(100000);
    
    let phone_x = 1;
    let phone_y = 2;
    let phone_z = 3;
    let phone_p = 4;
    
    // x -> y
    &roots.add_edge(phone_x, phone_y);
    
    // x -> z
    &roots.add_edge(phone_x, phone_z);
    
    // y -> x
    &roots.add_edge(phone_y, phone_x);

    // y -> z
    &roots.add_edge(phone_y, phone_z);

    // p -> x
    &roots.add_edge(phone_p, phone_x);

    assert_eq!(&Some (vec! [2, 3]), &roots.lookup(phone_x));
    assert_eq!(&Some (vec! [2, 4]), &roots.rlookup(phone_x));
}

fn init() -> Rc<RefCell<Node>> {
    let root = Node::new(1);

    let b = Node::new(2);
    let c = Node::new(3);
    let d = Node::new(4);
    let e = Node::new(5);
    let f = Node::new(6);

    {
        let mut mut_root = root.borrow_mut();
        let mut mut_b = b.borrow_mut();
        mut_b.followers.push(root.clone());
        mut_root.following.push(b.clone());

        mut_root.following.push(c.clone());
        mut_root.following.push(d.clone());

        let mut mut_c = c.borrow_mut();
        mut_c.following.push(e.clone());
        mut_c.following.push(f.clone());
        mut_c.following.push(root.clone());
    }

    root
}

pub fn _main_init() {
    let g = init();
    let g = g.borrow();
    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
    let f = g.first();
    foo(&*f.borrow());
}