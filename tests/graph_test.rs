// Need to 'use' an interface
use grapher::graph::{Graph, DirectedGraph, Node};

#[test]
fn debug_graph()
{
    let g = DirectedGraph::from([(1,2),(2,3),(3,0),(1,0)]);
    assert_eq!("1[2,0]2[3]3[0]0[]", &format!("{:?}", g));
}

#[test]
fn add_edge_allow_duplicates()
{
    let g = DirectedGraph::from([(0,1),(0,1)]);
    assert_eq!("0[1,1]1[]", &format!("{:?}", g));
}

#[test]
fn add_edge_circles()
{
    let g = DirectedGraph::from([(0,0)]);
    assert_eq!("0[0]", &format!("{:?}", g));
}

#[test]
fn iterate_nodes()
{
    let g = DirectedGraph::from([(0,1),(4,5)]);
    let node_list = [0, 1, 4, 5];
    for (index, node) in g.nodes().enumerate() {
        assert_eq!(g.get_value(node), &node_list[index]);
    }

}

#[test]
fn iterate_edges()
{
    let input_array = [(0,1),(2,3)];
    let g = DirectedGraph::from(input_array.clone());
    let edges = g.edges().collect::<Vec<(Node,Node)>>();
    for (i, (source, target)) in input_array.iter().enumerate() {
        assert_eq!(g.get_value(edges[i].0), source);
        assert_eq!(g.get_value(edges[i].1), target);
    }

}

#[test]
fn iterate_edges_empty_graph()
{
    let g: DirectedGraph<i32> = DirectedGraph::new();
    assert!(g.edges().next().is_none());
}

#[test]
fn graph_does_not_has_ghost_nodes()
{
    let g = DirectedGraph::from([(1,2)]);
    for node in g.nodes() {
        assert_ne!(g.get_value(node), &0);
    }
}

#[test]
fn graph_has_correct_size()
{
    let g = DirectedGraph::from([(1,2)]);
    assert_eq!(g.len(), 2);
}