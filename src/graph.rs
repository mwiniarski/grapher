use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::{fmt, hash::Hash};
use crate::directed::*;
use crate::graph_trait::*;
use std::iter::Iterator;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Node {
    uid: usize
}

impl Node {
    pub fn new() -> Self {
        Node { uid: std::usize::MAX }
    }

    fn from(node: GraphNode) -> Self {
        Node { uid: node.uid }
    }
}

impl GraphNode {
    fn from(node: Node) -> Self {
        GraphNode { uid: node.uid }
    }
}

pub struct NodeIterator<'a> {
    iterator: GraphNodeIterator<'a>
}

pub struct EdgeIterator<'a> {
    iterator: GraphEdgeIterator<'a>
}

pub struct Graph<T, U> {
    graph: U,
    values: Vec<T>
}

impl<T, U: GraphType> Graph<T, U> {
    // Create an unconnected node
    // O(1) amortized
    pub fn add_node(&mut self, value: T) -> Node {
        self.values.push(value);
        Node::from(self.graph.add_node())
    }

    // Add edge between two existing nodes
    // O(1)
    pub fn add_edge(&mut self, source: Node, target: Node) {
        self.graph.add_edge(GraphNode::from(source), GraphNode::from(target))
    }

    // Iterate over all nodes
    pub fn nodes(&self) -> NodeIterator {
        NodeIterator { iterator: self.graph.nodes() }
    }

    // Iterate over all edges
    pub fn edges(&self) -> EdgeIterator {
        EdgeIterator { iterator: self.graph.edges() }
    }

    // Get value associated with node
    // O(1)
    pub fn get_value(&self, node: Node) -> &T {
        &self.values[node.uid]
    }

    // Number of nodes
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    // Get a vector of neighbouring nodes
    pub fn get_neighbours(&self, node: Node) -> Vec<Node> {
        self.graph.get_neighbours(GraphNode::from(node))
            .iter()
            .map(|x| Node { uid: x.uid })
            .collect()
    }

    pub fn new() -> Self {
        Graph { graph: U::new(), values: Vec::new() }
    }
}

impl<T: Clone, U> Graph<T, U> {
    // Get value associated with both ends of the edge. Makes copies
    // O(1)
    pub fn get_value_edge(&self, edge: (Node, Node)) -> (T, T) {
        (self.values[edge.0.uid].clone(), self.values[edge.1.uid].clone())
    }
}

impl<T: fmt::Display, U: GraphType> Graph<T, U> {
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

impl<T: fmt::Display, U: GraphType> fmt::Display for Graph<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(true))
    }
}

impl<T: fmt::Display, U: GraphType> fmt::Debug for Graph<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(false))
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.iterator.next() {
            Some(node) => Some(Node::from(node)),
            None => None
        }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = (Node, Node);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.iterator.next() {
            Some((source, target)) => Some((Node::from(source), Node::from(target))),
            None => None
        }
    }
}

impl<T> Graph<T, Directed> {
    pub fn new_directed() -> Self {
        Graph { graph: Directed::new(), values: Vec::new() }
    }
}

impl<T : Eq + Hash + Clone, const N: usize> From<[(T, T); N]> for Graph<T, Directed> {
    fn from(arr: [(T, T); N]) -> Self {
        let mut map: HashMap<T, Node> = HashMap::new();
        let mut graph = Graph::new_directed();

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

impl<T : Eq + Hash + Clone> Graph<T, Directed> {

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    pub fn from_vec(vec: Vec<(T,T)>) -> Self {
        let mut map: HashMap<T, Node> = HashMap::new();
        let mut graph = Graph::new_directed();

        for (source, target) in vec {

            let source_node = match map.entry(source.clone()) {
                Occupied(entry) => entry.get().clone(),
                Vacant(entry) => {
                    let node = graph.add_node(source);
                    entry.insert(node);
                    node
                }
            };

            let target_node = match map.entry(target.clone()) {
                Occupied(entry) => entry.get().clone(),
                Vacant(entry) => {
                    let node = graph.add_node(target);
                    entry.insert(node);
                    node
                }
            };

            graph.add_edge(source_node, target_node);
        }

        graph
    }
}
