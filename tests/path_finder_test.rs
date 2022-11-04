use grapher::graph::{Graph, DirectedGraph};
use grapher::path_finder::PathFinder;

#[test]
fn find_shortest_path()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    g.add_edge(0,2);
    g.add_edge(2,3);
    g.add_edge(1,3);
    g.add_edge(1,4);
    g.add_edge(3,4);
    assert_eq!(Some(vec![0, 1, 4]), PathFinder::find_shortest_path(&g, 0, 4));
}

#[test]
fn find_shortest_path_same_node()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    assert_eq!(None, PathFinder::find_shortest_path(&g, 0, 0));

    g.add_edge(0,0);
    assert_eq!(Some(vec![0, 0]), PathFinder::find_shortest_path(&g, 0, 0));
}

#[test]
fn find_shortest_path_loop()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    g.add_edge(1,0);
    assert_eq!(Some(vec![0,1,0]), PathFinder::find_shortest_path(&g, 0, 0));
}

#[test]
fn find_shortest_path_disconnected()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    g.add_edge(2, 3);
    assert_eq!(None, PathFinder::find_shortest_path(&g, 0, 2));
}

#[test]
#[should_panic(expected="Source node doesn't exist")]
fn find_shortest_path_bad_node() 
{
    let g = DirectedGraph::new();
    PathFinder::find_shortest_path(&g, 0, 2);
}

#[test]
fn find_all_paths_two_nodes()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    assert_eq!(vec![vec![0,1]], PathFinder::find_all_paths(&g, 0, 1));
}

#[test]
fn find_all_paths_loops()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0, 0);
    g.add_edge(0, 1);
    g.add_edge(1, 0);
    assert_eq!(vec![vec![0,0], vec![0,1,0]], PathFinder::find_all_paths(&g, 0, 0));
}

#[test]
fn find_all_paths_line()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    g.add_edge(1,2);
    assert_eq!(vec![vec![0,1,2]], PathFinder::find_all_paths(&g, 0, 2));
}

#[test]
fn find_all_paths_two_paths()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    g.add_edge(1,2);
    g.add_edge(0,2);
    assert_eq!(vec![vec![0,1,2],vec![0,2]], PathFinder::find_all_paths(&g, 0, 2));
}

#[test]
fn find_all_paths_circle()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    g.add_edge(1,0);
    g.add_edge(0,2);
    assert_eq!(vec![vec![0,2]], PathFinder::find_all_paths(&g, 0, 2));
}

#[test]
fn find_all_paths_disconnected_graph()
{
    let mut g = DirectedGraph::new();
    g.add_edge(0,1);
    g.add_edge(2,3);
    assert_eq!(Vec::<Vec::<usize>>::new(), PathFinder::find_all_paths(&g, 0, 3));
}