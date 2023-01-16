use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::ops::{IndexMut, Index};
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

impl<T: Copy> Graph<T> {
    // Get tuple of values associated with edge
    // O(1)
    pub fn get_edge_values(&self, edge: (Node, Node)) -> (T, T) {
        (self.values[edge.0.uid], self.values[edge.1.uid])
    }
}

impl<T> Index<Node> for Graph<T> {
    type Output = T;

    fn index(&self, index: Node) -> &Self::Output {
        &self.values[index.uid]
    }
}

impl<T> IndexMut<Node> for Graph<T> {

    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.values[index.uid]
    }
}

impl<T: PartialEq> Graph<T> {
    pub fn find_node_with_value(&self, value: &T) -> Option<Node> {
        for node in self.nodes() {
            if &self[node] == value {
                return Some(node);
            }
        }
        None
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

impl<T: fmt::Display> Graph<T> {
    fn print(&self, pretty: bool) -> String {

        let mut output = String::new();
        for node in self.nodes() {
            output.push_str(&format!("{}[", self[node]));

            for (num_index, neighbour) in self.get_neighbours(node).into_iter().enumerate() {
                output.push_str(&self[neighbour].to_string());
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

impl<T> Graph<T> {
    pub fn new<U:GraphType + 'static>() -> Self {
        Graph { graph: Box::new(U::new()), values: Vec::new() }
    }
}

// Directed graph helpers
impl<T> Graph<T> {
    pub fn new_directed() -> Self {
        Graph { graph: Box::new(Directed::new()), values: Vec::new() }
    }
}

impl<T : Eq + Hash + Clone, const N: usize> From<[(T, T); N]> for Graph<T> {
    fn from(arr: [(T, T); N]) -> Self {
        Graph::fill_graph_from_vec(Graph::new_directed(), arr)
    }
}

impl<T : Eq + Hash + Clone> Graph<T> {

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    pub fn from_vec_directed(vec: Vec<(T,T)>) -> Self {
        Graph::fill_graph_from_vec(Graph::new_directed(), vec)
    }

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    pub fn from_vec_undirected(vec: Vec<(T,T)>) -> Self {
        Graph::fill_graph_from_vec(Graph::new_undirected(), vec)
    }

    fn fill_graph_from_vec<W : IntoIterator<Item = (T,T)>>(mut graph: Graph<T>, sth: W) -> Self {
        let mut map: HashMap<T, Node> = HashMap::new();

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

