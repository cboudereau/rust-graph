use rust_graph::graph::{GraphMap, GraphMapFeatures};
use std::time::Instant;

// cargo build --release && .\target\release\rust-graph.exe
pub fn main () {
    let now = Instant::now();    
    
    let mut graph = GraphMap::with_capacity(100_000_000);
    
    // Space problem (max 50M @ 50GB), timing are good though.
    
    // top: 50_000_000
    // [254385] initialized (+254s)
    // [254405] lookup: 50  (+20ms)
    // [254405] rlookup: 50 (+0ms)
    // [254461] suggest: 10 (+56ms)


    for i in 1..=50_000_000u32 {
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