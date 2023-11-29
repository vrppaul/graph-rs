mod graph;
use crate::graph::Graph;

#[macro_use]
mod macros;

fn main() {
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
    let graph = create_graph!(
        num_nodes: 20,
        edges: [
            0 -(3)-> 1,
            1 -(2)-> 3,
            3 -(4)-> 6,
            0 -(5)-> 2,
            1 -(3)-> 5,
            3 -(1)-> 8,
            6 -(7)-> 11,
            2 -(6)-> 5,
            5 -(7)-> 8,
            8 -(8)-> 11,
            2 -(2)-> 19,
            5 -(5)-> 4,
            11 -(4)-> 10,
            4 -(1)-> 7,
            7 -(3)-> 10,
            10 -(5)-> 13,
            7 -(2)-> 8,
            10 -(4)-> 12,
            13 -(9)-> 15,
            9 -(7)-> 12,
            12 -(9)-> 15,
            12 -(8)-> 16,
            15 -(8)-> 0,
            0 -(1)-> 16,
            16 -(1)-> 17,
            17 -(2)-> 18,
            18 -(3)-> 19,
            14 -(1)-> 16,
            14 -(6)-> 17,
            14 -(4)-> 18
        ]
    );

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
