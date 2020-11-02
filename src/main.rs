use rust_graph::graph::{GraphMap, GraphMapFeatures};
use std::time::Instant;

// cargo build --release && .\target\release\rust-graph.exe
pub fn main () {
    let now = Instant::now();    
    
    let mut graph = GraphMap::with_capacity(100_000_000);
    
    // Space problem (max 50M @ 50GB), timing are good though.
    
    // top: 50_000_000
    // [201512] initialized (+200s)
    // [201514] lookup: 50  (+12ms)
    // [201516] rlookup: 50 (+4ms)
    // [201525] suggest: 10 (+9ms)
    
    for i in 1..=50_000_000u32 {
        if i % 1000_000 == 0 {
            println!("top: {}", i);
        }
        let i_node = &graph.add_node(i);
        for j in 0..50 {
            if i > j {            
                let j_node = &graph.add_node(i-j);
                &graph.add_edge(i_node, j_node);
            } else {
                let j_node = &graph.add_node(i+j);
                &graph.add_edge(i_node, j_node);
            }
        }
    }

    println!("[{}] initialized", now.elapsed().as_millis());

    let l = &graph.lookup(1000).unwrap().len();

    println!("[{}] lookup: {:?}", now.elapsed().as_millis(), l);

    let rl = &graph.rlookup(1000).unwrap().len();

    println!("[{}] rlookup: {:?}", now.elapsed().as_millis(), rl);

    let sl = &graph.suggest(1000).unwrap().len();

    println!("[{}] suggest: {:?}", now.elapsed().as_millis(), sl);
}