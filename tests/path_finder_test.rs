use grapher::graph::{Node, Graph};
use grapher::path_finder::PathFinder;

fn compare_results<T: std::fmt::Debug + Eq, const N: usize>(graph: &Graph<T>, values: [T; N], nodes: Option<Vec<Node>>) {
    let nodes = nodes.unwrap();
    assert_eq!(values.len(), nodes.len());

    for (index, node) in nodes.iter().enumerate() {
        assert_eq!(values[index], graph[*node]);
    }
}

// O(n) way to get the node with value
fn n<T: Eq>(graph: &Graph<T>, value: T) -> Node {
    graph.find_node_with_value(&value).expect("Can't find node")
}

#[test]
fn find_shortest_path()
{
    let g = Graph::from([(0,1), (0,2), (2,3), (1,3), (1,4), (3,4)]);

    compare_results(&g, [0, 1, 4], 
        PathFinder::find_shortest_path(&g, 
            n(&g, 0), 
            n(&g, 4)));
}

#[test]
fn find_shortest_path_same_node()
{
    let mut g = Graph::from([(0,1)]);
    let node0 = n(&g, 0);
    assert_eq!(None, PathFinder::find_shortest_path(&g, node0, node0));

    g.add_edge(node0 , node0);
    compare_results(&g, [0, 0], 
        PathFinder::find_shortest_path(&g, node0, node0));
}

#[test]
fn find_shortest_path_loop()
{
    let g = Graph::from([(0,1), (1,0)]);
    let node0 = n(&g, 0);
    compare_results(&g, [0, 1, 0], 
        PathFinder::find_shortest_path(&g, node0, node0));
}

#[test]
fn find_shortest_path_disconnected()
{
    let g = Graph::from([(0,1),(2,3)]);
    assert_eq!(None, PathFinder::find_shortest_path(&g, n(&g, 0), n(&g, 2)));
}

#[test]
fn find_all_paths_two_nodes()
{
    let g = Graph::from([(0,1)]);
    let node0 = n(&g, 0);
    let node1 = n(&g, 1);
    assert_eq!(vec![vec![node0, node1]], PathFinder::find_all_paths(&g, node0, node1));
}

#[test]
fn find_all_paths_loops()
{
    let g = Graph::from([(0,0), (0,1), (1,0)]);
    let node0 = n(&g, 0);
    let node1 = n(&g, 1);
    assert_eq!(vec![vec![node0,node0], vec![node0,node1,node0]], PathFinder::find_all_paths(&g, node0, node0));
}

#[test]
fn find_all_paths_line()
{
    let g = Graph::from([(0,1), (1,2)]);
    let node0 = n(&g, 0);
    let node1 = n(&g, 1);
    let node2 = n(&g, 2);
    assert_eq!(vec![vec![node0,node1,node2]], PathFinder::find_all_paths(&g, node0, node2));
}

#[test]
fn find_all_paths_two_paths()
{
    let g = Graph::from([(0,1), (1,2), (0,2)]);
    let node0 = n(&g, 0);
    let node1 = n(&g, 1);
    let node2 = n(&g, 2);
    assert_eq!(vec![vec![node0,node1,node2],vec![node0,node2]], PathFinder::find_all_paths(&g, node0, node2));
}

#[test]
fn find_all_paths_circle()
{
    let g = Graph::from([(0,1), (1,0), (0,2)]);
    let node0 = n(&g, 0);
    let node2 = n(&g, 2);
    assert_eq!(vec![vec![node0,node2]], PathFinder::find_all_paths(&g, node0, node2));
}

#[test]
fn find_all_paths_disconnected_graph()
{
    let g = Graph::from([(0,1), (2,3)]);
    assert_eq!(Vec::<Vec::<Node>>::new(), PathFinder::find_all_paths(&g, n(&g, 0), n(&g, 3)));
}