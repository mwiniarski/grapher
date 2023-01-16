// It should be easy to swap out your own GraphType
//
// 1. Import traits

use std::iter;

use grapher::graph_trait::{GraphType, GraphNode, GraphEdgeIterator, GraphNodeIterator};
use grapher::graph::*;

// 2. Define a new GraphType

struct MyGraphType {
    storage: Vec<Vec<usize>>
}

// 3. Define iterators

struct MyGraphNodeIterator<'a> {
    graph: &'a MyGraphType,
    index: usize
}

struct MyGraphEdgeIterator<'a> {
    graph: &'a MyGraphType,
    index: (usize, NeighbourIterator<'a>)
}

// 4. Implement the trait 
//
// MyGraph is a directed full graph, there exist two directed edges between every pair of nodes

impl GraphType for MyGraphType {
    fn add_node(&mut self) -> GraphNode {
        let new_index = self.storage.len();
        self.storage.push(Vec::new());

        for i in 0..new_index {
            self.storage[i].push(new_index);
            self.storage[new_index].push(i);
        }
        GraphNode { uid: new_index }
    }

    fn add_edge(&mut self, _source: GraphNode, _target: GraphNode) {
        panic!("Add edge can't be used for this graph")
    }

    fn nodes(&self) -> GraphNodeIterator {
        GraphNodeIterator { iterator: Box::new( MyGraphNodeIterator { index: 0, graph: self }) }
    }

    fn edges(&self) -> GraphEdgeIterator {
        GraphEdgeIterator { iterator: Box::new( MyGraphEdgeIterator { index: (0, NeighbourIterator{ iterator: Box::new(iter::empty::<&usize>()) }), graph: self }) }
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn get_neighbours(&self, node: GraphNode) -> GraphNodeIterator {
        GraphNodeIterator{ 
            iterator: Box::new( 
                NeighbourIterator{ 
                    iterator: Box::new(self.storage[node.uid].iter())
                }
            )
        }
    }

    fn get_degree(&self, node: GraphNode) -> usize {
        self.storage[node.uid].len()
    }

    fn new() -> Self {
        MyGraphType { storage: Vec::new() }
    }
}

// 5. Implement iterators

impl<'a> Iterator for MyGraphNodeIterator<'a> {
    type Item = GraphNode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.graph.len() {
            return None
        }

        let ret = Some(GraphNode{uid:self.index});
        self.index += 1;
        ret
    }
}

impl Iterator for MyGraphEdgeIterator<'_> {
    type Item = (GraphNode, GraphNode);

    fn next(&mut self) -> Option<Self::Item> { 
        return match self.get_next_existing_edge() {
            Some(target_node) => Some((GraphNode{uid:self.index.0}, target_node)),
            None => None
        }
    }
}

impl MyGraphEdgeIterator<'_> {
    fn get_next_existing_edge(&mut self) -> Option<GraphNode> {
        loop {
            match self.index.1.next() {
                Some(value) => return Some(value),
                None => ()
            };

            self.index.0 += 1;
            if self.index.0 < self.graph.len() {
                return None;
            }
            
            self.index.1.iterator = Box::new(self.graph.storage[self.index.0].iter());
        }
    }
}

pub struct NeighbourIterator<'a> {
    iterator: Box<dyn Iterator<Item=&'a usize> + 'a>
}

impl<'a> Iterator for NeighbourIterator<'a> {
    type Item = GraphNode;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(index) => Some(GraphNode{uid:index.clone()}),
            None => None,
        }
    }
}

#[test]
fn its_possible_to_create_graph_and_use_methods()
{
    let mut graph: Graph<usize> = Graph::new::<MyGraphType>();
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);

    let theoretical_edges = [(1,2), (1,3), (2,1), (2,3), (3,1), (3,2)];
    for (i, edge) in graph.edges().enumerate() {
        assert_eq!(theoretical_edges[i], graph.get_edge_values(edge));
    }
}