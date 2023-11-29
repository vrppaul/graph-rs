#[macro_export]
macro_rules! create_graph {
    (num_nodes: $num_nodes:expr, edges: [$($start:literal -($weight:literal)-> $end:literal), *]) => {{
        let mut graph = Graph::new();
        for i in 0..$num_nodes {
            graph.add_node(i);
        }
        // print all edges
        $(
            graph.add_edge($start, $end, $weight).unwrap();
        )*
        graph
    }};
}
