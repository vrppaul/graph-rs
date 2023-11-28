use graph_rs::Graph;

fn main() {
    let mut graph = Graph::new();
    let node1 = graph.add_node(11);
    let node2 = graph.add_node(22);
    let node3 = graph.add_node(33);
    let node4 = graph.add_node(44);
    let node5 = graph.add_node(55);
    let node6 = graph.add_node(66);
    graph.add_edge(node1, node2, 1).unwrap();
    graph.add_edge(node1, node3, 1).unwrap();
    graph.add_edge(node2, node4, 1).unwrap();
    graph.add_edge(node4, node5, 1).unwrap();
    graph.add_edge(node3, node6, 2).unwrap();
    graph.show();
}
