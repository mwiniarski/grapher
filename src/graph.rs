use std::{fmt, collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Node {
    uid: usize
}

impl Node {
    pub fn new() -> Self {
        Node { uid: std::usize::MAX }
    }
}

pub trait Graph<T> {
    // Create an unconnected node
    // O(1) amortized
    fn add_node(&mut self, value: T) -> Node;

    // Add edge between two existing nodes
    // O(1)
    fn add_edge(&mut self, source: Node, target: Node);

    // Check if node exists in this graph
    // O(1)
    fn node_exists(&self, node: Node) -> bool;

    // Iterate over all nodes
    fn nodes(&self) -> NodeIterator<T>;

    // Get value associated with node
    // O(1)
    fn get_value(&self, node: Node) -> &T;

    // Number of nodes
    fn size(&self) -> usize;

    // Get a vector of neighbouring nodes
    fn get_neighbours(&self, node: Node) -> Vec<Node>;
}

struct InternalNode<T> {
    value: T,
    index: usize,
    neighbours: Vec<usize>
}

pub struct DirectedGraph<T> {
    adjacency_list: Vec<InternalNode<T>>,
}

impl<T : Eq + Hash + Clone, const N: usize> From<[(T, T); N]> for DirectedGraph<T> {

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    fn from(arr: [(T, T); N]) -> Self {
        let mut map: HashMap<T, Node> = HashMap::new();
        let mut graph: DirectedGraph<T> = DirectedGraph { adjacency_list: Vec::new() };

        for (source, target) in arr {
            let source_node: Node;
            let target_node: Node;
            if !map.contains_key(&source) {
                let cloned_value = source.clone();
                source_node = graph.add_node(source);
                map.insert(cloned_value, source_node);
            }
            else {
                source_node = map[&source];
            }

            if !map.contains_key(&target) {
                let cloned_value = target.clone();
                target_node = graph.add_node(target);
                map.insert(cloned_value, target_node);
            }
            else {
                target_node = map[&target];
            }

            graph.add_edge(source_node, target_node);
        }

        graph
    }
}

impl<T> Graph<T> for DirectedGraph<T> {

    fn add_node(&mut self, value: T) -> Node {
        let new_node_index = self.adjacency_list.len();
        self.adjacency_list.push(InternalNode { value, index: new_node_index, neighbours: Vec::new() });
        Node {uid: new_node_index}
    }

    fn add_edge(&mut self, source: Node, target: Node) {

        for node in vec![source, target] {
            if !self.node_exists(node) {
                panic!("Node doesn't exist");
            }
        }

        self.adjacency_list[source.uid].neighbours.push(target.uid);
    }

    fn get_value(&self, node: Node) -> &T {
        &self.adjacency_list[node.uid].value
    }

    fn get_neighbours(&self, node: Node) -> Vec<Node> {
        if !self.node_exists(node) {
            panic!("Node doesn't exist!")
        }

        let mut neighbour_nodes = Vec::new();
        for neighbour_index in self.adjacency_list[node.uid].neighbours.iter() {
            neighbour_nodes.push(Node { uid: self.adjacency_list[*neighbour_index].index });
        }

        neighbour_nodes
    }

    fn node_exists(&self, node: Node) -> bool {
        self.node_exists_index(node.uid)
    }

    fn nodes(&self) -> NodeIterator<T> {
        NodeIterator{graph: self, current_node_index: 0}
    }

    fn size(&self) -> usize {
        self.adjacency_list.len()
    }
}

impl<T> DirectedGraph<T> {

    pub fn new() -> Self {
        DirectedGraph { adjacency_list: Vec::new() }
    }

    pub fn edges(&self) -> EdgeIterator<T> {
        EdgeIterator{graph: self, current_node_index: 0, current_target_index: 0}
    }

    fn node_exists_index(&self, index: usize) -> bool {
        index < self.adjacency_list.len()
    }
}

impl<T: std::fmt::Display> DirectedGraph<T> {
    fn print(&self, pretty: bool) -> String {

        let mut s = String::new();
        for row in self.adjacency_list.iter() {

            s.push_str(&format!("{}[", row.value));

            for (num_index, neighbour) in row.neighbours.iter().enumerate() {
                s.push_str(&self.adjacency_list[*neighbour].value.to_string());
                if num_index < row.neighbours.len() - 1 {
                    s.push(',');
                }
            }
            s.push_str(&format!("]{}", if pretty { "\n" } else { "" }));
        }
        s
    }
}

impl<T: std::fmt::Display> fmt::Display for DirectedGraph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(true))
    }
}

impl<T: std::fmt::Display> fmt::Debug for DirectedGraph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(false))
    }
}

pub struct NodeIterator<'a, T> {
    graph: &'a DirectedGraph<T>,
    current_node_index: usize
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.graph.node_exists_index(self.current_node_index) {
            return None
        }

        let ret = Some( Node { uid: self.current_node_index });
        self.current_node_index += 1;
        ret
    }
}

pub struct EdgeIterator<'a, T> {
    graph: &'a DirectedGraph<T>,
    current_node_index: usize,
    current_target_index: usize
}

impl<T> Iterator for EdgeIterator<'_, T> {
    type Item = (Node, Node);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.find_next_existing_edge() {
            return None;
        }

        let ret = Some((
            Node { uid: self.current_node_index },
            Node { uid: self.graph.adjacency_list[self.current_node_index].neighbours[self.current_target_index]}
        ));

        self.current_target_index += 1;
        ret
    }
}

impl<T> EdgeIterator<'_, T> {
    fn find_next_existing_edge(&mut self) -> bool {
        while self.graph.node_exists_index(self.current_node_index) {
            if self.current_target_index < self.graph.adjacency_list[self.current_node_index].neighbours.len() {
                return true;
            }
            self.current_node_index += 1;
            self.current_target_index = 0;
        }
        false
    }
}