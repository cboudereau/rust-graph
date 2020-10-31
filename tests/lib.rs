
#[cfg(test)]
mod tests {
    use rust_graph::graph::{GraphMap, GraphMapFeatures};
    use std::collections::HashSet;
    
    #[test]
    fn graph_features_tests() {
        let mut graph = GraphMap::with_capacity(100000);
        
        let x = 1;
        let y = 2;
        let z = 3;
        let p = 4;
        
        // x -> y
        &graph.add_edge(x, y);
        
        // x -> z
        &graph.add_edge(x, z);
        
        // y -> x
        &graph.add_edge(y, x);

        // y -> z
        &graph.add_edge(y, z);

        // p -> x
        &graph.add_edge(p, x);

        // assert x
        assert_eq!(&Some (vec! [y, z]), &graph.lookup(x));
        assert_eq!(&Some (vec! [y, p]), &graph.rlookup(x));

        // assert y
        assert_eq!(&Some (vec! [x, z]), &graph.lookup(y));       
        assert_eq!(&Some (vec! [x]), &graph.rlookup(y));       

        // assert z
        assert_eq!(&Some (vec! []), &graph.lookup(z));
        assert_eq!(&Some (vec! [x, y]), &graph.rlookup(z));

        // assert p
        assert_eq!(&Some (vec! [x]), &graph.lookup(p));
        assert_eq!(&Some (vec! []), &graph.rlookup(p));

        // p can follow y and z since p -> x -> y -> z
        assert_eq!(&Some(vec!(y, z).into_iter().collect()), &graph.suggest(p).map(|x| x.into_iter().collect::<HashSet<_>>()));
    }
}