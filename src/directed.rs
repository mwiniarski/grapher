use crate::adjacency_list;
use crate::graph_trait::{GraphType, GraphNode, GraphEdgeIterator, GraphNodeIterator};

pub struct Directed {
    adjacency_list: adjacency_list::AdjancencyList,
}

impl GraphType for Directed {
    fn add_node(&mut self) -> GraphNode {
        self.adjacency_list.add_node();
        GraphNode {uid: self.adjacency_list.len() - 1}
    }

    fn add_edge(&mut self, source: GraphNode, target: GraphNode) {
        self.adjacency_list.add_edge(source.uid, target.uid);
    }

    fn get_neighbours(&self, node: GraphNode) -> Vec<GraphNode> {
        let neighbours = self.adjacency_list.get_neighbours(node.uid);
        let mut ret = Vec::with_capacity(neighbours.len());

        for neighbour_index in neighbours {
            ret.push(GraphNode { uid: *neighbour_index });
        }
        ret
    }

    fn nodes(&self) -> GraphNodeIterator {
        GraphNodeIterator{ iterator: Box::new( NodeIterator{ iterator: self.adjacency_list.nodes() } ) }
    }

    fn edges(&self) -> GraphEdgeIterator {
        GraphEdgeIterator { iterator: Box::new( EdgeIterator{ iterator: self.adjacency_list.edges() } ) }
    }

    fn len(&self) -> usize {
        self.adjacency_list.len()
    }

    fn new() -> Self {
        Directed { 
            adjacency_list: adjacency_list::AdjancencyList::new()
        }
    }
}

pub struct NodeIterator<'a> {
    iterator: adjacency_list::NodeIterator<'a>
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = GraphNode;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(index) => Some(GraphNode { uid: index }),
            None => None
        }
    }
}

pub struct EdgeIterator<'a> {
    iterator: adjacency_list::EdgeIterator<'a>
}

impl Iterator for EdgeIterator<'_> {
    type Item = (GraphNode, GraphNode);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some((source, target)) => Some((
                GraphNode { uid: source },
                GraphNode { uid: target }
            )),
            None => None
        }
    }
}