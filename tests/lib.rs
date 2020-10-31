
#[cfg(test)]
mod tests {
    use rust_graph::graph::Roots;
    use rust_graph::graph::RootsFeatures;
    
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
}