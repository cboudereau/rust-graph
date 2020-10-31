
#[cfg(test)]
mod tests {
    use rust_graph::graph::{GraphMap, GraphMapFeatures};
    use std::collections::HashSet;
    
    #[test]
    fn from_sample_lookup_and_rlookup() {
        let mut graph = GraphMap::with_capacity(100000);
        
        let phone_x = 1;
        let phone_y = 2;
        let phone_z = 3;
        let phone_p = 4;
        
        // x -> y
        &graph.add_edge(phone_x, phone_y);
        
        // x -> z
        &graph.add_edge(phone_x, phone_z);
        
        // y -> x
        &graph.add_edge(phone_y, phone_x);

        // y -> z
        &graph.add_edge(phone_y, phone_z);

        // p -> x
        &graph.add_edge(phone_p, phone_x);

        // assert x
        assert_eq!(&Some (vec! [2, 3]), &graph.lookup(phone_x));
        assert_eq!(&Some (vec! [2, 4]), &graph.rlookup(phone_x));

        // assert y
        assert_eq!(&Some (vec! [1, 3]), &graph.lookup(phone_y));       
        assert_eq!(&Some (vec! [1]), &graph.rlookup(phone_y));       

        // assert z
        assert_eq!(&Some (vec! []), &graph.lookup(phone_z));
        assert_eq!(&Some (vec! [1, 2]), &graph.rlookup(phone_z));

        // assert p
        assert_eq!(&Some (vec! [1]), &graph.lookup(phone_p));
        assert_eq!(&Some (vec! []), &graph.rlookup(phone_p));

        // p can follow x and y since p -> x -> y (2) -> z (3)
        assert_eq!(&Some(vec!(2, 3).into_iter().collect()), &graph.suggest(phone_p).map(|x| x.into_iter().collect::<HashSet<_>>()));
    }
}