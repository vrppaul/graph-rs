use std::cmp::{self, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Eq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering here
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.cost == other.cost
    }
}

pub struct Graph<T> {
    nodes: Vec<GraphNode<T>>,
}

impl<T: Debug> Graph<T> {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, value: T) -> usize {
        let id = self.nodes.len();
        let node = GraphNode::new(id, value);
        self.nodes.insert(id, node);
        id
    }

    fn get_node(&self, id: usize) -> Option<&GraphNode<T>> {
        self.nodes.get(id)
    }

    pub fn add_edge(&mut self, from_id: usize, to_id: usize, weight: usize) -> Result<(), String> {
        if from_id >= self.nodes.len() {
            return Err(format!("Node with ID {} does not exist", from_id));
        }

        if to_id >= self.nodes.len() {
            return Err(format!("Node with ID {} does not exist", to_id));
        }

        let edge = GraphEdge {
            into: to_id,
            weight,
        };

        let from_node = self.nodes.get_mut(from_id).unwrap();
        from_node.edges.insert(edge);
        Ok(())
    }

    pub fn dfs(&self, start_id: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut stack: Vec<usize> = Vec::new();

        // Put the start into stack and go through all its adjucent nodes
        stack.push(start_id);

        while let Some(node_id) = stack.pop() {
            if visited.contains(&node_id) {
                continue;
            }
            visited.insert(node_id);
            path.push(node_id);

            let node = self.get_node(node_id).unwrap();
            let mut adjucent_node_ids: Vec<usize> =
                node.edges.iter().map(|edge| edge.into).collect();
            adjucent_node_ids.sort();

            for &adjucent_node_id in adjucent_node_ids.iter().rev() {
                if visited.contains(&adjucent_node_id) {
                    continue;
                }
                stack.push(adjucent_node_id);
            }
        }

        path
    }

    pub fn bfs(&self, start_id: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();

        queue.push_front(start_id);
        visited.insert(start_id);

        while let Some(node_id) = queue.pop_back() {
            path.push(node_id);

            let node = self.get_node(node_id).unwrap();
            let mut adjucent_node_ids: Vec<usize> =
                node.edges.iter().map(|edge| edge.into).collect();
            adjucent_node_ids.sort();

            for adjucent_node_id in adjucent_node_ids {
                if visited.contains(&adjucent_node_id) {
                    continue;
                }

                queue.push_front(adjucent_node_id);
                visited.insert(adjucent_node_id);
            }
        }
        path
    }

    pub fn dijkstra_dist(&self, start_id: usize, to_id: usize) -> (usize, Vec<usize>) {
        let mut previous: Vec<Option<usize>> = vec![None; self.nodes.len()];
        let mut distances: Vec<usize> = vec![usize::MAX; self.nodes.len()];
        let mut heap = BinaryHeap::new();

        distances[start_id] = 0;
        heap.push(State {
            cost: 0,
            position: start_id,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == to_id {
                break;
            }
            if cost > distances[position] {
                continue;
            }

            for edge in &self.nodes[position].edges {
                let next = State {
                    cost: cost + edge.weight,
                    position: edge.into,
                };

                if next.cost < distances[next.position] {
                    distances[next.position] = next.cost;
                    previous[next.position] = Some(position);
                    heap.push(next);
                }
            }
        }

        let mut path = Vec::new();
        let mut current_id = to_id;
        while let Some(prev_id) = previous[current_id] {
            path.push(current_id);
            current_id = prev_id;
        }
        path.push(start_id);
        path.reverse();
        (distances[to_id], path)
    }

    pub fn show(&self) {
        let mut shown_nodes: HashSet<usize> = HashSet::new();
        for id in 0..self.nodes.len() {
            if !shown_nodes.contains(&id) {
                self.show_node(id, &mut shown_nodes, 0);
            }
        }
    }

    fn show_node(&self, id: usize, shown_nodes: &mut HashSet<usize>, depth: usize) {
        if let Some(node) = self.get_node(id) {
            shown_nodes.insert(id); // Mark the node as shown

            // Print the node with indentation
            println!("{}┌─────────┐", " ".repeat(depth * 4));
            println!("{}│ Node {:2} │", " ".repeat(depth * 4), id);
            println!("{}│ {:<7?} │", " ".repeat(depth * 4), node.value);
            println!("{}└─────────┘", " ".repeat(depth * 4));

            // Handle connected nodes
            if !node.edges.is_empty() {
                for edge in &node.edges {
                    let edge_desc = if shown_nodes.contains(&edge.into) {
                        format!("Back to Node {}", edge.into) // Indicate a cycle
                    } else {
                        format!("Node {}", edge.into)
                    };
                    println!(
                        "{}├─[{}]─> {}",
                        " ".repeat(depth * 4),
                        edge.weight,
                        edge_desc
                    );

                    // Recursively show unvisited connected nodes
                    if !shown_nodes.contains(&edge.into) {
                        self.show_node(edge.into, shown_nodes, depth + 1);
                    }
                }
            } else {
                println!("{}└─", " ".repeat(depth * 4));
            }
        } else {
            println!(
                "{}Node with ID {} does not exist",
                " ".repeat(depth * 4),
                id
            );
        }
    }
}

impl<T: Debug> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct GraphEdge {
    // The ID of the node this edge points to
    into: usize,
    // The weight of this edge
    weight: usize,
}

impl Hash for GraphEdge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.into.hash(state);
        self.weight.hash(state);
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state)
        }
    }
}

impl PartialEq for GraphEdge {
    fn eq(&self, other: &Self) -> bool {
        self.into == other.into && self.weight == other.weight
    }
}

impl Eq for GraphEdge {}

struct GraphNode<T> {
    id: usize,
    value: T,
    edges: HashSet<GraphEdge>,
}

impl<T> GraphNode<T> {
    fn new(id: usize, value: T) -> Self {
        GraphNode {
            id,
            value,
            edges: HashSet::new(),
        }
    }
}

impl<T> Hash for GraphNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for GraphNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for GraphNode<T> {}

impl<T> Debug for GraphNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Node id: {}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    fn prepare_graph() -> Graph<usize> {
        // Graph will have the following structure:
        //
        // 0 -(1)-> 1 -(1)-> 3 -(1)-> 4 -(1)-> 0
        // |        |                 ^
        // |(1)     |(3)              | (1)
        // v        v                 |
        // 2 -(2)-> 5 ----------------+
        //
        let mut graph = Graph::new();
        let node0 = graph.add_node(11);
        let node1 = graph.add_node(22);
        let node2 = graph.add_node(33);
        let node3 = graph.add_node(44);
        let node4 = graph.add_node(55);
        let node5 = graph.add_node(66);
        graph.add_edge(node0, node1, 1).unwrap();
        graph.add_edge(node0, node2, 1).unwrap();
        graph.add_edge(node1, node3, 1).unwrap();
        graph.add_edge(node1, node5, 3).unwrap();
        graph.add_edge(node3, node4, 1).unwrap();
        graph.add_edge(node2, node5, 2).unwrap();
        graph.add_edge(node5, node4, 1).unwrap();
        graph.add_edge(node4, node0, 1).unwrap();
        graph
    }

    #[test]
    fn correct_dfs() {
        let graph = prepare_graph();
        assert_eq!(graph.dfs(0), [0, 1, 3, 4, 5, 2]);
    }

    #[test]
    fn correct_bfs() {
        let graph = prepare_graph();
        assert_eq!(graph.bfs(0), [0, 1, 2, 3, 5, 4]);
    }

    #[test]
    fn correct_dijkstra_dist() {
        let graph = prepare_graph();
        assert_eq!(graph.dijkstra_dist(0, 5), (3, vec![0, 2, 5]));
    }
}
