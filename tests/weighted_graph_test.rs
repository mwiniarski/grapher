use grapher::weighted_graph::WeightedGraph;

#[test]
fn weighted_graph_can_add_weighted_edge()
{
    let mut g = WeightedGraph::new_directed();
    let n1 = g.add_node(0);
    let n2 = g.add_node(1);
    g.add_edge(n1, n2, 0.5);
    //assert_eq!(g.get_neighbours(n1).weight, 0.5);
}