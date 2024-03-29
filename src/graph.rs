use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::ops::{IndexMut, Index};
use std::{fmt, hash::Hash};
use crate::graph_trait::*;
use crate::path_finder::PathFindable;
use crate::weighted_graph::*;

pub type Node = crate::weighted_graph::Node;
pub type Edge = crate::weighted_graph::Edge;
pub type NodeIter<'a, T> = crate::weighted_graph::NodeIter<'a, T>;

struct EmptyWeight;

pub struct Graph<T> {
    graph: WeightedGraph<T, EmptyWeight>
}

impl<T> Graph<T> {
    // Create an unconnected node
    // O(1) amortized
    pub fn add_node(&mut self, value: T) -> Node {
        self.graph.add_node(value)
    }

    // Add edge between two existing nodes
    // O(1)
    pub fn add_edge(&mut self, source: Node, target: Node) {
        self.graph.add_edge(source, target, EmptyWeight)
    }

    // Iterate over all nodes
    pub fn nodes(&self) -> NodeIter<T> {
        self.graph.nodes()
    }

    // Iterate over all edges
    pub fn edges(&self) -> EdgeIter {
        EdgeIter { iterator: self.graph.edges() }
    }

    // Number of nodes
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    // Get a vector of neighbouring nodes
    pub fn get_neighbours(&self, node: Node) -> EdgeIter {
        EdgeIter { iterator: self.graph.get_neighbours(node) }
    }

    pub fn get_degree(&self, node: Node) -> usize {
        self.graph.get_degree(node)
    }
}

impl<T: Copy> Graph<T> {
    // Get tuple of values associated with edge
    // O(1)
    pub fn get_edge_values(&self, edge: Edge) -> (T, T) {
        self.graph.get_edge_values(edge)
    }
}

impl<T> Index<Node> for Graph<T> {
    type Output = T;

    fn index(&self, index: Node) -> &Self::Output {
        &self.graph[index]
    }
}

impl<T> IndexMut<Node> for Graph<T>  {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.graph[index]
    }
}

impl<T: PartialEq> Graph<T> {
    pub fn find_node_with_value(&self, value: &T) -> Option<Node> {
        self.graph.find_node_with_value(value)
    }
}

pub struct EdgeIter<'a> {
    iterator: crate::weighted_graph::EdgeIter<'a, EmptyWeight>
}

impl<'a> Iterator for EdgeIter<'a> {
    type Item = Edge;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some((edge, _)) => Some(edge),
            None => None
        }
    }
}

impl<T: fmt::Display> Graph<T> {
    fn print(&self, pretty: bool) -> String {

        let mut output = String::new();
        for node in self.nodes() {
            output.push_str(&format!("{}[", self[node.0]));

            for (num_index, neighbour) in self.get_neighbours(node.0).into_iter().enumerate() {
                output.push_str(&self[neighbour.target].to_string());
                if num_index < self.get_degree(node.0) - 1 {
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
    pub fn new<U: GraphType + 'static>() -> Self {
        Graph { graph: WeightedGraph::new::<U>() }
    }
}

// Directed graph helpers
impl<T> Graph<T> {
    pub fn new_directed() -> Self {
        Graph { graph: WeightedGraph::new_directed() }
    }
}

impl<T> Graph<T> {
    pub fn new_undirected() -> Self {
        Graph { graph: WeightedGraph::new_undirected() }
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

impl<'a, T> PathFindable<'a, Node, usize> for Graph<T> {
    fn nodes(&'a self) -> Box<dyn Iterator<Item=Node> + 'a> {
        Box::new(self.nodes().map(|(node, _)| node))
    }

    fn get_neighbours(&'a self, n: Node) -> Box<dyn Iterator<Item=(Node, usize)> + 'a> {
        Box::new(self.get_neighbours(n).map(|edge| (edge.target, 1)))
    }
}