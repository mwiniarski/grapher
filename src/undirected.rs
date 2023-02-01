use crate::adjacency_list;
use crate::graph_trait::{GraphType, GraphNode, GraphEdge, GraphEdgeIterator, GraphNodeIterator};

pub struct Undirected {
    adjacency_list: adjacency_list::AdjancencyList
}

impl GraphType for Undirected {
    fn add_node(&mut self) -> GraphNode {
        self.adjacency_list.add_node();
        self.adjacency_list.len() - 1
    }

    fn add_edge(&mut self, source: GraphNode, target: GraphNode, edge_index: usize) {
        self.adjacency_list.add_edge(source, target, edge_index);
        self.adjacency_list.add_edge(target, source, edge_index);
    }

    fn get_neighbours(&self, node: GraphNode) -> GraphEdgeIterator {
        GraphEdgeIterator{
            iterator: Box::new(
                self.adjacency_list
                    .get_neighbours(node)
                    .iter()
                    .map(move |conn| GraphEdge { source: node, target: conn.node_index.clone(), uid: conn.edge_index.clone() })
            )
        }
    }

    fn get_degree(&self, node: GraphNode) -> usize {
        self.adjacency_list.get_neighbours(node).len()
    }

    fn nodes(&self) -> GraphNodeIterator {
        GraphNodeIterator {
            iterator: Box::new(
                self.adjacency_list
                    .nodes()
                    .map(|index| index )
        )}
    }

    fn edges(&self) -> GraphEdgeIterator {
        GraphEdgeIterator {
            iterator: Box::new(
                self.adjacency_list
                    .edges()
                    .map(|(source, target, uid)| GraphEdge{source, target, uid})
        )}
    }

    fn len(&self) -> usize {
        self.adjacency_list.len()
    }

    fn new() -> Self {
        Undirected { 
            adjacency_list: adjacency_list::AdjancencyList::new()
        }
    }
}