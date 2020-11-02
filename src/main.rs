use rust_graph::graph::{GraphMap, GraphMapFeatures};
use std::time::Instant;

// cargo build --release && .\target\release\rust-graph.exe
pub fn main () {
    let now = Instant::now();    
    
    let mut graph = GraphMap::with_capacity(100_000_000);
    
    // Space problem (max 30M), timing are good though.
    
    // 20M
    // [116008] initialized
    // [116010] lookup: 50  (+2ms)
    // [116011] rlookup: 50 (+1ms)
    // [116016] suggest: 10 (+5ms)

    // 30M
    // [160088] initialized
    // [160093] lookup: 50  (+5ms)
    // [160093] rlookup: 50 (+0ms)
    // [160114] suggest: 10 (+9ms)

    for i in 1..=10_000_000u32 {
        if i % 1000_000 == 0 {
            println!("top: {}", i);
        }

        for j in 0..50 {
            if i > j {            
                &graph.add_edge(i, i-j);
            } else {
                &graph.add_edge(i, i+j);
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