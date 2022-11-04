// Need to 'use' an interface
use grapher::graph::{Graph, DirectedGraph};

#[test]
fn debug_graph()
{
    let mut g = DirectedGraph::new();
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    g.add_edge(3, 0);
    g.add_edge(1, 0);
    assert_eq!("0[]1[2,0]2[3]3[0]", &format!("{:?}", g));
}

#[test]
fn add_edge_allow_duplicates()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    g.add_edge(0,1);
    assert_eq!("0[1,1]1[]", &format!("{:?}", g));
}

#[test]
fn add_edge_circles()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,0);
    assert_eq!("0[0]", &format!("{:?}", g));
}