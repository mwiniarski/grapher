use crate::adjacency_list;
use crate::graph_trait::{GraphType, GraphNode, GraphEdgeIterator, GraphNodeIterator};

pub struct Directed<Idx> {
    adjacency_list: adjacency_list::AdjancencyList<Idx>,
}

impl<Idx> GraphType<Idx> for Directed<Idx> {
    fn add_node(&mut self) -> GraphNode {
        self.adjacency_list.add_node();
        GraphNode {uid: self.adjacency_list.len() - 1}
    }

    fn add_edge(&mut self, source: GraphNode, target: GraphNode, edge_index: Idx) {
        self.adjacency_list.add_edge(source.uid, target.uid, edge_index)
    }

    fn get_neighbours(&self, node: GraphNode) -> GraphNodeIterator {
        GraphNodeIterator{
            iterator: Box::new(
                self.adjacency_list
                    .get_neighbours(node.uid)
                    .iter()
                    .map(|conn| GraphNode { uid: conn.node_index.clone() })
            )
        }
    }

    fn nodes(&self) -> GraphNodeIterator {
        GraphNodeIterator {
            iterator: Box::new(
                self.adjacency_list
                    .nodes()
                    .map(|index| GraphNode { uid: index })
        )}
    }

    fn edges(&self) -> GraphEdgeIterator {
        GraphEdgeIterator {
            iterator: Box::new(
                self.adjacency_list
                    .edges()
                    .map(|(source, target)| (GraphNode{uid:source}, GraphNode{uid:target}))
        )}
    }

    fn len(&self) -> usize {
        self.adjacency_list.len()
    }

    fn get_degree(&self, node: GraphNode) -> usize {
        self.adjacency_list.get_neighbours(node.uid).len()
    }

    fn new() -> Self {
        Directed { 
            adjacency_list: adjacency_list::AdjancencyList::new()
        }
    }
}