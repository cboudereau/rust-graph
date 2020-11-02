use rust_graph::graph::{GraphMap, GraphMapFeatures};
use std::time::Instant;

// cargo build --release && .\target\release\rust-graph.exe
pub fn main () {
    let now = Instant::now();    
    
    let mut graph = GraphMap::with_capacity(100_000_000);
    
    // Space problem (max 50M @ 50GB), timing are good though.
    
    // top: 50000000
    // [+180594ms] initialized
    // [+2ms] lookup: 50
    // [+0ms] rlookup: 50
    // [+9ms] suggest: 10
    
    for i in 1..=50_000_000u64 {
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
    let initialized = now.elapsed().as_millis();
    println!("[+{}ms] initialized", &initialized);

    let l = &graph.lookup(1000).unwrap().len();
    let lookup = now.elapsed().as_millis() - initialized;
    println!("[+{}ms] lookup: {:?}", &lookup, l);

    let rl = &graph.rlookup(1000).unwrap().len();
    let rlookup = now.elapsed().as_millis() - initialized - lookup;
    println!("[+{}ms] rlookup: {:?}", &rlookup, rl);

    let sl = &graph.suggest(1000).unwrap().len();
    let suggest = now.elapsed().as_millis() - initialized - lookup - rlookup;
    println!("[+{}ms] suggest: {:?}", &suggest, sl);
}