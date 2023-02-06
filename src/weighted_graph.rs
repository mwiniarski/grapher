use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::ops::{IndexMut, Index};
use std::{fmt, hash::Hash};

use crate::directed::Directed;
use crate::{graph_trait::*};
use crate::path_finder::PathFindable;
use crate::undirected::Undirected;
use std::iter::{Iterator};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Node {
    pub(crate) uid: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Edge {
    pub source: Node,
    pub target: Node,
    pub(crate) uid: usize
}

pub struct WeightedGraph<T, W> {
    graph: Box<dyn GraphType>,
    values: Vec<T>,
    weights: Vec<W>,
    edge_count: usize
}

impl<T, W> WeightedGraph<T, W> {
    // Create an unconnected node
    // O(1) amortized
    pub fn add_node(&mut self, value: T) -> Node {
        self.values.push(value);
        Node::from(self.graph.add_node())
    }

    // Add edge between two existing nodes
    // O(1)
    pub fn add_edge(&mut self, source: Node, target: Node, weight: W) {
        self.graph.add_edge(
            source.uid,
            target.uid,
            self.edge_count);
        self.edge_count += 1;
        self.weights.push(weight);
    }

    // Iterate over all nodes
    pub fn nodes(&self) -> NodeIter<T> {
        NodeIter { iterator: self.graph.nodes(), values: self.values.as_ptr() }
    }

    pub fn nodes_mut(&mut self) -> NodeIterMut<T> {
        NodeIterMut { iterator: self.graph.nodes(), values: self.values.as_mut_ptr() }
    }

    // Iterate over all edges
    pub fn edges(&self) -> EdgeIter<W> {
        EdgeIter { iterator: self.graph.edges(), weights: self.weights.as_ptr() }
    }

    pub fn edges_mut(&mut self) -> EdgeIterMut<W> {
        EdgeIterMut { iterator: self.graph.edges(), weights: self.weights.as_mut_ptr() }
    } 

    // Number of nodes
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    // Get a vector of neighbouring nodes
    pub fn get_neighbours(&self, node: Node) -> EdgeIter<W> {
        EdgeIter { iterator: self.graph.get_neighbours(node.uid), weights: self.weights.as_ptr() }
    }

    pub fn get_degree(&self, node: Node) -> usize {
        self.graph.get_degree(node.uid)
    }

    pub fn get_weight(&self, edge: Edge) -> &W {
        &self.weights[edge.uid]
    }
}

impl<T: Copy, W> WeightedGraph<T, W> {
    // Get tuple of values associated with edge
    // O(1)
    pub fn get_edge_values(&self, edge: Edge) -> (T, T) {
        (self.values[edge.source.uid], self.values[edge.target.uid])
    }
}

impl<T, W> Index<Node> for WeightedGraph<T, W> {
    type Output = T;

    fn index(&self, index: Node) -> &Self::Output {
        &self.values[index.uid]
    }
}

impl<T, W> IndexMut<Node> for WeightedGraph<T, W> {

    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self.values[index.uid]
    }
}

impl<T: PartialEq, W> WeightedGraph<T, W> {
    pub fn find_node_with_value(&self, value: &T) -> Option<Node> {
        for node in self.nodes() {
            if node.1 == value {
                return Some(node.0);
            }
        }
        None
    }
}

impl<T: fmt::Debug, W: fmt::Debug> WeightedGraph<T, W> {
    fn print(&self, pretty: bool) -> String {

        let mut output = String::new();
        for node in self.nodes() {
            // 0{value}[
            output.push_str(&format!("{:?}{{{:?}}}[", node.0.uid, node.1));

            for (num_index, neighbour) in self.get_neighbours(node.0).enumerate() {
                // 1(weight)
                output.push_str(&format!("{:?}({:?})", neighbour.0.target.uid, neighbour.1));
                if num_index < self.get_degree(node.0) - 1 {
                    output.push(',');
                }
            }
            output.push_str(&format!("]{}", if pretty { "\n" } else { "" }));
        }
        output
    }
}

// impl<T: fmt::Display, W: fmt::Display> fmt::Display for WeightedGraph<T, W> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.print(true))
//     }
// }

impl<T: fmt::Debug, W: fmt::Debug> fmt::Debug for WeightedGraph<T, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(true))
    }
}

impl<T, W> WeightedGraph<T, W> {
    pub fn new<U: GraphType + 'static>() -> Self {
        WeightedGraph { graph: Box::new(U::new()), values: Vec::new(), weights: Vec::new(), edge_count: 0 }
    }
}

// Directed graph helpers
impl<T, W> WeightedGraph<T, W> {
    pub fn new_directed() -> Self {
        WeightedGraph { graph: Box::new(Directed::new()), values: Vec::new(), weights: Vec::new(), edge_count: 0 }
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
        WeightedGraph { graph: Box::new(Undirected::new()), values: Vec::new(), weights: Vec::new(), edge_count: 0 }
    }
}

impl Node {
    pub fn new() -> Self { Node { uid: GraphNode::MAX } }
    pub fn from(node: GraphNode) -> Self { Node { uid: node } }
}

impl Edge {
    pub fn from(edge: GraphEdge) -> Self { 
        Edge { source: Node { uid: edge.source }, 
               target: Node { uid: edge.target }, 
               uid: edge.uid } 
        }
}

pub struct NodeIter<'a, T> {
    iterator: GraphNodeIterator<'a>,
    values: *const T
}

pub struct NodeIterMut<'a, T> {
    iterator: GraphNodeIterator<'a>,
    values: *mut T
}

pub struct EdgeIter<'a, W> {
    iterator: GraphEdgeIterator<'a>,
    weights: *const W
}

pub struct EdgeIterMut<'a, W> {
    iterator: GraphEdgeIterator<'a>,
    weights: *mut W
}

impl<'a, T: 'a> Iterator for NodeIter<'a, T> {
    type Item = (Node, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.iterator.next() {
            Some(i) => {
                unsafe {
                    // SAFETY: i is always a valid index for values vector
                    let ptr = self.values.offset(i as isize);
                    Some((Node::from(i), &*ptr))
                }
            },
            None => None,
        }
    }
}

impl<'a, T: 'a> Iterator for NodeIterMut<'a, T> {
    type Item = (Node, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.iterator.next() {
            Some(i) => {
                unsafe {
                    // SAFETY: i is always a valid index for values vector
                    let ptr = self.values.offset(i as isize);
                    Some((Node::from(i), &mut *ptr))
                }
            },
            None => None,
        }
    }
}

impl<'a, W: 'a> Iterator for EdgeIter<'a, W> {
    type Item = (Edge, &'a W);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.iterator.next() {
            Some(edge) => {
                unsafe {
                    // SAFETY: i is always a valid index for values vector
                    let ptr = self.weights.offset(edge.uid as isize);
                    Some((Edge::from(edge), &*ptr))
                }
            },
            None => None,
        }
    }
}

impl<'a, W: 'a> Iterator for EdgeIterMut<'a, W> {
    type Item = (Edge, &'a mut W);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.iterator.next() {
            Some(edge) => {
                unsafe {
                    // SAFETY: i is always a valid index for values vector
                    let ptr = self.weights.offset(edge.uid as isize);
                    Some((Edge::from(edge), &mut *ptr))
                }
            },
            None => None,
        }
    }
}

impl<'a, T, W: Copy> PathFindable<'a, Node, W> for WeightedGraph<T, W> {
    fn nodes(&'a self) -> Box<dyn Iterator<Item=Node> + 'a> {
        Box::new(self.nodes().map(|(node, _)| node))
    }

    fn get_neighbours(&'a self, n: Node) -> Box<dyn Iterator<Item=(Node, W)> + 'a> {
        Box::new(self.get_neighbours(n).map(|edge| (edge.0.target, *edge.1)))
    }
}
