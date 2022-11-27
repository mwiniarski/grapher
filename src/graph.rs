use std::{fmt, collections::HashMap, hash::Hash};
use crate::adjacency_list;

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

    // Iterate over all nodes
    fn nodes(&self) -> NodeIterator;

    // Iterate over all edges
    fn edges(&self) -> EdgeIterator;

    // Get value associated with node
    // O(1)
    fn get_value(&self, node: Node) -> &T;

    // Number of nodes
    fn len(&self) -> usize;

    // Get a vector of neighbouring nodes
    fn get_neighbours(&self, node: Node) -> Vec<Node>;
}

pub struct DirectedGraph<T> {
    adjacency_list: adjacency_list::AdjancencyList,
    values: Vec<T>
}

impl<T : Eq + Hash + Clone, const N: usize> From<[(T, T); N]> for DirectedGraph<T> {

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    fn from(arr: [(T, T); N]) -> Self {
        let mut map: HashMap<T, Node> = HashMap::new();
        let mut graph: DirectedGraph<T> = DirectedGraph { 
            adjacency_list: adjacency_list::AdjancencyList::new(),
            values: Vec::new() 
        };

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

        self.adjacency_list.add_node();
        self.values.push(value);

        Node {uid: self.adjacency_list.len() - 1}
    }

    fn add_edge(&mut self, source: Node, target: Node) {
        self.adjacency_list.add_edge(source.uid, target.uid);
    }

    fn get_value(&self, node: Node) -> &T {
        &self.values[node.uid]
    }

    fn get_neighbours(&self, node: Node) -> Vec<Node> {

        let mut neighbour_nodes = Vec::new();
        for neighbour_index in self.adjacency_list.get_neighbours(node.uid) {
            neighbour_nodes.push(Node { uid: *neighbour_index });
        }

        neighbour_nodes
    }

    fn nodes(&self) -> NodeIterator {
        NodeIterator{ iterator: self.adjacency_list.nodes() }
    }

    fn edges(&self) -> EdgeIterator {
        EdgeIterator { iterator: self.adjacency_list.edges() }
    }

    fn len(&self) -> usize {
        self.adjacency_list.len()
    }
}

impl<T> DirectedGraph<T> {

    pub fn new() -> Self {
        DirectedGraph { 
            adjacency_list: adjacency_list::AdjancencyList::new(),
            values: Vec::new() 
        }
    }
}

impl<T: std::fmt::Display> DirectedGraph<T> {
    fn print(&self, pretty: bool) -> String {

        let mut output = String::new();
        for node in self.nodes() {
            output.push_str(&format!("{}[", self.get_value(node)));

            for (num_index, neighbour) in self.get_neighbours(node).into_iter().enumerate() {
                output.push_str(&self.get_value(neighbour).to_string());
                if num_index < self.get_neighbours(node).len() - 1 {
                    output.push(',');
                }
            }
            output.push_str(&format!("]{}", if pretty { "\n" } else { "" }));
        }
        output
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

pub struct NodeIterator<'a> {
    iterator: adjacency_list::NodeIterator<'a>
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(index) => Some(Node { uid: index }),
            None => None
        }
    }
}

pub struct EdgeIterator<'a> {
    iterator: adjacency_list::EdgeIterator<'a>
}

impl Iterator for EdgeIterator<'_> {
    type Item = (Node, Node);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some((source, target)) => Some((
                Node { uid: source },
                Node { uid: target }
            )),
            None => None
        }
    }
}