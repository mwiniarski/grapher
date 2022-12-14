// It should be easy to swap out your own GraphType
//
// 1. Import traits

use grapher::graph_trait::{GraphType, GraphNode, GraphEdgeIterator, GraphNodeIterator};
use grapher::graph::*;

// 2. Define a new GraphType

struct MyGraphType {
    storage: Vec<Vec<GraphNode>>
}

// 3. Define iterators

struct MyGraphNodeIterator<'a> {
    graph: &'a MyGraphType,
    index: GraphNode
}

struct MyGraphEdgeIterator<'a> {
    graph: &'a MyGraphType,
    index: (GraphNode, GraphNode)
}

// 4. Implement the trait 
//
// MyGraph is a directed full graph, there exist two directed edges between every pair of nodes

impl GraphType for MyGraphType {
    fn add_node(&mut self) -> GraphNode {
        let new_index = self.storage.len();
        self.storage.push(Vec::new());

        for i in 0..new_index {
            self.storage[i].push(GraphNode{uid:new_index});
            self.storage[new_index].push(GraphNode{uid:i});
        }
        GraphNode { uid: new_index }
    }

    fn add_edge(&mut self, _source: GraphNode, _target: GraphNode) {
        panic!("Add edge can't be used for this graph")
    }

    fn nodes(&self) -> GraphNodeIterator {
        GraphNodeIterator { iterator: Box::new( MyGraphNodeIterator { index: GraphNode{uid:0}, graph: self }) }
    }

    fn edges(&self) -> GraphEdgeIterator {
        GraphEdgeIterator { iterator: Box::new( MyGraphEdgeIterator { index: (GraphNode{uid:0}, GraphNode{uid:0}), graph: self }) }
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn get_neighbours(&self, node: GraphNode) -> Vec<GraphNode> {
        self.storage[node.uid].clone()
    }

    fn new() -> Self {
        MyGraphType { storage: Vec::new() }
    }
}

// 5. Implement iterators

impl<'a> Iterator for MyGraphNodeIterator<'a> {
    type Item = GraphNode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index.uid >= self.graph.len() {
            return None
        }

        let ret = Some(self.index);
        self.index.uid += 1;
        ret
    }
}

impl Iterator for MyGraphEdgeIterator<'_> {
    type Item = (GraphNode, GraphNode);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.find_next_existing_edge() {
            return None;
        }

        let ret = Some((
            self.index.0,
            self.graph.get_neighbours(self.index.0)[self.index.1.uid]
        ));

        self.index.1.uid += 1;
        ret
    }
}

impl MyGraphEdgeIterator<'_> {
    fn find_next_existing_edge(&mut self) -> bool {
        while self.index.0.uid < self.graph.len() {
            if self.index.1.uid < self.graph.get_neighbours(self.index.0).len() {
                return true;
            }
            self.index.0.uid += 1;
            self.index.1.uid = 0;
        }
        false
    }
}

#[test]
fn its_possible_to_create_graph_and_use_methods()
{
    let mut graph: Graph<usize, MyGraphType> = Graph::new();
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);

    let theoretical_edges = [(1,2), (1,3), (2,1), (2,3), (3,1), (3,2)];
    for (i, edge) in graph.edges().enumerate() {
        assert_eq!(theoretical_edges[i], graph.get_value_edge(edge));
    }
}