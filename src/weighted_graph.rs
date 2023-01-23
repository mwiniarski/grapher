use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::ops::{IndexMut, Index};
use std::{fmt, hash::Hash};
use num_traits::NumAssign;

use crate::graph::{Node, Graph, NodeIterator, EdgeIterator};
use crate::graph_trait::*;
use crate::path_finder::PathFindable;
use std::iter::Iterator;


pub struct WeightedGraph<T, W, Idx = u32> where Idx: num_traits::Unsigned + num_traits::NumAssign + Copy {
    graph: Graph<T, Idx>,
    weights: Vec<W>
}

impl<T, W, Idx> WeightedGraph<T, W, Idx> where Idx: num_traits::Unsigned + NumAssign + Copy {
    // Create an unconnected node
    // O(1) amortized
    pub fn add_node(&mut self, value: T) -> Node {
        self.graph.add_node(value)
    }

    // Add edge between two existing nodes
    // O(1)
    pub fn add_edge(&mut self, source: Node, target: Node, weight: W) {
        self.graph.add_edge(source, target);
        self.weights.push(weight);
    }

    // Iterate over all nodes
    pub fn nodes(&self) -> NodeIterator {
        self.graph.nodes()
    }

    // Iterate over all edges
    pub fn edges(&self) -> EdgeIterator {
        self.graph.edges()
    }

    // Number of nodes
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    // Get a vector of neighbouring nodes
    pub fn get_neighbours(&self, node: Node) -> NodeIterator {
        self.graph.get_neighbours(node)
    }

    pub fn get_degree(&self, node: Node) -> usize {
        self.graph.get_degree(node)
    }
}

impl<T, W> Index<Node> for WeightedGraph<T, W> {
    type Output = T;

    fn index(&self, index: Node) -> &Self::Output {
        &self.graph[index]
    }
}

impl<T, W> IndexMut<Node> for WeightedGraph<T, W> {

    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.graph[index]
    }
}

impl<T: PartialEq, W> WeightedGraph<T, W> {
    pub fn find_node_with_value(&self, value: &T) -> Option<Node> {
        for node in self.nodes() {
            if &self[node] == value {
                return Some(node);
            }
        }
        None
    }
}

impl<T: fmt::Display, W> WeightedGraph<T, W> {
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

impl<T: fmt::Display, W> fmt::Display for WeightedGraph<T, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(true))
    }
}

impl<T: fmt::Display, W> fmt::Debug for WeightedGraph<T, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(false))
    }
}

impl<T, W, Idx> WeightedGraph<T, W, Idx> where Idx: num_traits::Unsigned + num_traits::NumAssign + Copy {
    pub fn new<U: GraphType<Idx> + 'static>() -> Self {
        WeightedGraph { graph: Graph::new::<U>(), weights: Vec::new() }
    }
}

// Directed graph helpers
impl<T, W> WeightedGraph<T, W> {
    pub fn new_directed() -> Self {
        WeightedGraph { graph: Graph::new_directed(), weights: Vec::new() }
    }
}

impl<T : Eq + Hash + Clone, const N: usize, W> From<[(T, T, W); N]> for WeightedGraph<T, W> {
    fn from(arr: [(T, T, W); N]) -> Self {
        WeightedGraph::fill_graph_from_vec(WeightedGraph::new_directed(), arr)
    }
}

impl<T : Eq + Hash + Clone, W> WeightedGraph<T, W> {

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    pub fn from_vec_directed(vec: Vec<(T, T, W)>) -> Self {
        WeightedGraph::fill_graph_from_vec(WeightedGraph::new_directed(), vec)
    }

    // Constructs graph
    // O(N) time
    // O(unique vertex count) size
    pub fn from_vec_undirected(vec: Vec<(T, T, W)>) -> Self {
        WeightedGraph::fill_graph_from_vec(WeightedGraph::new_undirected(), vec)
    }

    fn fill_graph_from_vec<I : IntoIterator<Item = (T, T, W)>>(mut graph: WeightedGraph<T, W>, sth: I) -> Self {
        let mut map: HashMap<T, Node> = HashMap::new();

        for (source, target, weight) in sth {

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

            graph.add_edge(source_node, target_node, weight);
        }

        graph
    }
}

impl<T, W> WeightedGraph<T, W> {
    pub fn new_undirected() -> Self {
        WeightedGraph { graph: Graph::new_undirected(), weights: Vec::new() }
    }
}

impl<'a, T, W: num_traits::PrimInt> PathFindable<'a, W> for WeightedGraph<T, W> {
    fn nodes(&'a self) -> NodeIterator<'a> {
        self.nodes()
    }

    fn get_neighbours(&'a self, n: Node) -> Box<dyn Iterator<Item=(Node, W)> + 'a> {
        Box::new(self.get_neighbours(n).map(|node|(node, W::one())))
    }
}