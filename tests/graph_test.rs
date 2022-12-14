// Need to 'use' an interface
use grapher::graph::{Graph, Node};

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
        assert_eq!(g.get_value(node), &node_list[index]);
    }

}

#[test]
fn iterate_edges()
{
    let input_array = [(0,1),(2,3),(2,1)];
    let g = Graph::from(input_array);
    let edges = g.edges().collect::<Vec<(Node,Node)>>();
    for (i, edge) in input_array.iter().enumerate() {
        assert_eq!(g.get_value_edge(edges[i]), *edge);
    }
}

#[test]
fn iterate_edges_empty_graph()
{
    let g: Graph<usize, _> = Graph::new_directed();
    assert!(g.edges().next().is_none());
}

#[test]
fn graph_does_not_has_ghost_nodes()
{
    let g = Graph::from([(1,2)]);
    for node in g.nodes() {
        assert_ne!(g.get_value(node), &0);
    }
}

#[test]
fn graph_has_correct_size()
{
    let g = Graph::from([(1,2)]);
    assert_eq!(g.len(), 2);
}