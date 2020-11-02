use rust_graph::graph::{GraphMap, GraphMapFeatures};
use std::time::Instant;

pub fn main () {
    let now = Instant::now();    
    
    let mut graph = GraphMap::with_capacity(100_000_000);

    for i in 0..100_000_00u64 {
        for j in 0..50 {
            if i > j {
                &graph.add_edge(i, i-j);
            } else {
                &graph.add_edge(i, i+j);
            }
        }
    }

    println!("[{}] initialized", now.elapsed().as_millis());

    let l = &graph.lookup(1).unwrap().len();

    println!("[{}] lookup: {:?}", now.elapsed().as_millis(), l);

    let rl = &graph.rlookup(1).unwrap().len();

    println!("[{}] rlookup: {:?}", now.elapsed().as_millis(), rl);
}