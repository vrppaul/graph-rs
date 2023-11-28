use graph_rs::Graph;

fn main() {
    let mut graph = Graph::new();
    // Graph will have the following structure:
    // 0 -(3)-> 1 -(2)-> 3 -(4)-> 6
    // |        |        |        |
    // |(5)     |(3)     |(1)     | (7)
    // v        v        v        v
    // 2 -(6)-> 5 -(7)-> 8 -(8)-> 11
    // |        |        ^        |
    // |(2)     |(5)     |(2)     | (4)
    // |        v        |        v
    // |        4 -(1)-> 7 -(3)-> 10 -(5)-> 13
    // |                 |        |        |
    // |                 |(6)     |(4)     | (9)
    // |                 v        v        v
    // |                 9 -(7)-> 12 -(9)-> 15
    // |                          |        |
    // |                          |(8)     | (8)
    // v                          +--v     +---v
    // 19 <-(3)- 18 <-(2)- 17 <-(1)- 16 <-(1)- 0
    //            ^        ^        ^
    //            |(4)     |(6)     | (1)
    //            -----------<---- 14
    for i in 0..20 {
        graph.add_node(i);
    }
    graph.add_edge(0, 1, 3).unwrap();
    graph.add_edge(1, 3, 2).unwrap();
    graph.add_edge(3, 6, 4).unwrap();
    graph.add_edge(0, 2, 5).unwrap();
    graph.add_edge(1, 5, 3).unwrap();
    graph.add_edge(3, 8, 1).unwrap();
    graph.add_edge(6, 11, 7).unwrap();
    graph.add_edge(2, 5, 6).unwrap();
    graph.add_edge(5, 8, 7).unwrap();
    graph.add_edge(8, 11, 8).unwrap();
    graph.add_edge(2, 19, 2).unwrap();
    graph.add_edge(5, 4, 5).unwrap();
    graph.add_edge(11, 10, 4).unwrap();
    graph.add_edge(4, 7, 1).unwrap();
    graph.add_edge(7, 10, 3).unwrap();
    graph.add_edge(10, 13, 5).unwrap();
    graph.add_edge(7, 8, 2).unwrap();
    graph.add_edge(10, 13, 5).unwrap();
    graph.add_edge(7, 9, 6).unwrap();
    graph.add_edge(10, 12, 4).unwrap();
    graph.add_edge(13, 15, 9).unwrap();
    graph.add_edge(9, 12, 7).unwrap();
    graph.add_edge(12, 15, 9).unwrap();
    graph.add_edge(12, 16, 8).unwrap();
    graph.add_edge(15, 0, 8).unwrap();
    graph.add_edge(0, 16, 1).unwrap();
    graph.add_edge(16, 17, 1).unwrap();
    graph.add_edge(17, 18, 2).unwrap();
    graph.add_edge(18, 19, 3).unwrap();
    graph.add_edge(18, 19, 3).unwrap();
    graph.add_edge(14, 16, 1).unwrap();
    graph.add_edge(14, 17, 6).unwrap();
    graph.add_edge(14, 18, 4).unwrap();

    graph.show();

    let dfs = graph.dfs(0);
    println!("DFS: {:?}", dfs);
    let bfs = graph.bfs(0);
    println!("BFS: {:?}", bfs);

    const N: usize = 10000;

    // measure time of dfs for N runs
    let mut time = 0;
    for _ in 0..N {
        let start = std::time::Instant::now();
        graph.dfs(0);
        time += start.elapsed().as_micros();
    }
    println!("Time of dfs for {} runs: {} microseconds", N, time);

    // measure time of bfs for N runs
    let mut time = 0;
    for _ in 0..N {
        let start = std::time::Instant::now();
        graph.bfs(0);
        time += start.elapsed().as_micros();
    }
    println!("Time of bfs for {} runs: {} microseconds", N, time);
}
