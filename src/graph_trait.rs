
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct GraphNode {
    pub uid: usize
}

pub struct GraphNodeIterator<'a> {
    pub iterator: Box<dyn Iterator<Item=GraphNode> + 'a>
}

pub struct GraphEdgeIterator<'a> {
    pub iterator: Box<dyn Iterator<Item = (GraphNode,GraphNode)> + 'a>
}

pub trait GraphType {
    // Add a new node
    // O(1)
    fn add_node(&mut self) -> GraphNode;
    
    // Add edge between two existing nodes
    // O(1)
    fn add_edge(&mut self, source: GraphNode, target: GraphNode);

    // Iterate over all nodes
    fn nodes(&self) -> GraphNodeIterator;

    // Iterate over all edges
    fn edges(&self) -> GraphEdgeIterator;

    // Number of nodes
    fn len(&self) -> usize;

    // Get a vector of neighbouring nodes
    fn get_neighbours(&self, node: GraphNode) -> Vec<GraphNode>;
}