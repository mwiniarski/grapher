use grapher::weighted_graph::WeightedGraph;

#[test]
fn weighted_graph_can_add_weighted_edge()
{
    let mut g = WeightedGraph::new_directed();
    let n1 = g.add_node(0);
    let n2 = g.add_node(1);
    g.add_edge(n1, n2, 0.5);
    let e1 = g.get_neighbours(n1).next().unwrap();
    assert_eq!(g.get_weight(e1.0), &0.5);
    assert_eq!(e1.1, &0.5);
}

#[test]
fn debug_graph()
{
    let g = WeightedGraph::from([(1,2,"a"),(2,3,"b"),(3,0,"c"),(1,0,"d")]);
    assert_eq!("0{1}[1(a),3(d)]1{2}[2(b)]2{3}[3(c)]3{0}[]", &format!("{:?}", g));
}