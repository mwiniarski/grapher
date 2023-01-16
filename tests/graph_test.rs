// Need to 'use' an interface
use grapher::graph::{Graph, Node, NodeIterator};

#[test]
fn debug_graph()
{
    let g = Graph::from([(1,2),(2,3),(3,0),(1,0)]);
    assert_eq!("1[2,0]2[3]3[0]0[]", &format!("{:?}", g));
}

#[test]
fn add_edge_allow_duplicates()
{
    let g = Graph::from([(0,1),(0,1)]);
    assert_eq!("0[1,1]1[]", &format!("{:?}", g));
}

#[test]
fn add_edge_circles()
{
    let g = Graph::from([(0,0)]);
    assert_eq!("0[0]", &format!("{:?}", g));
}

#[test]
fn iterate_nodes()
{
    let g = Graph::from([(0,1),(4,5)]);
    let node_list = [0, 1, 4, 5];
    for (index, node) in g.nodes().enumerate() {
        assert_eq!(g[node], node_list[index]);
    }

}

#[test]
fn iterate_edges()
{
    let input_array = [(0,1),(2,3),(2,1)];
    let g = Graph::from(input_array);
    let edges = g.edges().collect::<Vec<(Node,Node)>>();
    for (i, edge) in input_array.iter().enumerate() {
        assert_eq!(g.get_edge_values(edges[i]), *edge);
    }
}

#[test]
fn iterate_edges_empty_graph()
{
    let g: Graph<usize> = Graph::new_directed();
    assert!(g.edges().next().is_none());
}

#[test]
fn graph_does_not_has_ghost_nodes()
{
    let g = Graph::from([(1,2)]);
    for node in g.nodes() {
        assert_ne!(g[node], 0);
    }
}

#[test]
fn graph_has_correct_size()
{
    let g = Graph::from([(1,2)]);
    assert_eq!(g.len(), 2);
}

#[test]
fn get_neighbours()
{
    let g = Graph::from([(1u32,2), (1,3), (1,4)]);
    let node = g.find_node_with_value(&1).expect("Node not found");
    let mut iter: NodeIterator = g.get_neighbours(node);
    assert_eq!(iter.next(), g.find_node_with_value(&2));
    assert_eq!(iter.next(), g.find_node_with_value(&3));
    assert_eq!(iter.next(), g.find_node_with_value(&4));
}

#[test]
fn create_directed_graph_from_vector()
{
    let v = vec![(1,2),(3,4),(1,5),(2,4)];
    let edges = vec![(1,2),(1,5),(2,4),(3,4)];
    let g = Graph::from_vec_directed(v);
    assert_eq!(g.edges().map(|edge| g.get_edge_values(edge)).collect::<Vec<(i32,i32)>>(), edges);
}