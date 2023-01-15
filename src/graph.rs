use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::{fmt, hash::Hash};
use crate::directed::*;
use crate::graph_trait::*;
use crate::undirected::*;
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

pub struct Graph<T> {
    graph: Box<dyn GraphType>,
    values: Vec<T>
}

impl<T> Graph<T> {
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
    pub fn get_neighbours(&self, node: Node) -> NodeIterator {
        NodeIterator {
            iterator: self.graph.get_neighbours(GraphNode::from(node))
        }
    }

    pub fn get_degree(&self, node: Node) -> usize {
        self.graph.get_degree(GraphNode::from(node))
    }
}

impl<T: Clone> Graph<T> {
    // Get value associated with both ends of the edge. Makes copies
    // O(1)
    pub fn get_value_edge(&self, edge: (Node, Node)) -> (T, T) {
        (self.values[edge.0.uid].clone(), self.values[edge.1.uid].clone())
    }
}

impl<T: PartialEq> Graph<T> {
    pub fn find_node_with_value(&self, value: &T) -> Option<Node> {
        for node in self.nodes() {
            if self.get_value(node) == value {
                return Some(node);
            }
        }
        None
    }
}

impl<T: fmt::Display> Graph<T> {
    fn print(&self, pretty: bool) -> String {

        let mut output = String::new();
        for node in self.nodes() {
            output.push_str(&format!("{}[", self.get_value(node)));

            for (num_index, neighbour) in self.get_neighbours(node).into_iter().enumerate() {
                output.push_str(&self.get_value(neighbour).to_string());
                if num_index < self.get_degree(node) - 1 {
                    output.push(',');
                }
            }
            output.push_str(&format!("]{}", if pretty { "\n" } else { "" }));
        }
        output
    }
}

impl<T: fmt::Display> fmt::Display for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(true))
    }
}

impl<T: fmt::Display> fmt::Debug for Graph<T> {
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

impl<T> Graph<T> {
    pub fn new_directed() -> Self {
        Graph { graph: Box::new(Directed::new()), values: Vec::new() }
    }
}

impl<T> Graph<T> {
    pub fn new<U:GraphType + 'static>() -> Self {
        Graph { graph: Box::new(U::new()), values: Vec::new() }
    }
}

impl<T : Eq + Hash + Clone, const N: usize> From<[(T, T); N]> for Graph<T> {
    fn from(arr: [(T, T); N]) -> Self {
        Graph::from_directed(arr)
    }
}

impl<T : Eq + Hash + Clone> Graph<T> {

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    pub fn from_vec_directed(vec: Vec<(T,T)>) -> Self {
        Graph::from_directed(vec)
    }

    fn from_directed<W : IntoIterator<Item = (T,T)>>(sth: W) -> Self {
        let mut map: HashMap<T, Node> = HashMap::new();
        let mut graph = Graph::new_directed();

        for (source, target) in sth {

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

impl<T> Graph<T> {
    pub fn new_undirected() -> Self {
        Graph { graph: Box::new(Undirected::new()), values: Vec::new() }
    }
}

impl<T : Eq + Hash + Clone> Graph<T> {

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    pub fn from_vec_undirected(vec: Vec<(T,T)>) -> Self {
        Graph::from_undirected(vec)
    }

    fn from_undirected<W : IntoIterator<Item = (T,T)>>(sth: W) -> Self {
        let mut map: HashMap<T, Node> = HashMap::new();
        let mut graph = Graph::new_undirected();

        for (source, target) in sth {

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

