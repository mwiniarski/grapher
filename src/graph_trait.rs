
pub type GraphNode = usize;

pub struct GraphEdge {
    pub source: usize,
    pub target: usize,
    pub uid: usize
}

pub struct GraphNodeIterator<'a> {
    pub iterator: Box<dyn Iterator<Item = GraphNode> + 'a>
}

pub struct GraphEdgeIterator<'a> {
    pub iterator: Box<dyn Iterator<Item = GraphEdge> + 'a>
}

pub trait GraphType {
    // Add a new node
    // O(1)
    fn add_node(&mut self) -> GraphNode;
    
    // Add edge between two existing nodes
    // O(1)
    fn add_edge(&mut self, source: GraphNode, target: GraphNode, edge_index: usize);

    // Iterate over all nodes
    fn nodes(&self) -> GraphNodeIterator;

    // Iterate over all edges
    fn edges(&self) -> GraphEdgeIterator;

    // Number of nodes
    fn len(&self) -> usize;

    // Number of edges leaving the node
    fn get_degree(&self, node: GraphNode) -> usize;

    // Get a vector of neighbouring nodes
    fn get_neighbours(&self, node: GraphNode) -> GraphEdgeIterator;

    fn new() -> Self where Self:Sized;
}