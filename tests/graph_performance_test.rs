use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Instant;

use grapher::directed::Directed;
use grapher::graph::*;

fn load_graph() -> std::io::Result<Graph<usize,Directed>> {
    let file = File::open("tests/data/facebook_test_data.txt")?;
    let buf_reader = BufReader::new(file);
    let mut temp_vec = vec![];

    for line in buf_reader.lines() {
        let nodes = 
            line.as_ref()
                .expect("Can't read line")
                .split_whitespace()
                .map(|n| n.parse::<usize>()
                                .expect("Can't parse node"))
                .collect::<Vec<usize>>();
       
        temp_vec.push((nodes[0], nodes[1]));
    }

    let time = Instant::now();
    let g = Graph::from_vec(temp_vec);
    println!("Graph::from_vec : {:.2?}", time.elapsed());
    
    assert!(time.elapsed().as_millis() < 80);
    Ok(g)
}

#[test]
fn performance_test1() {
    let graph = load_graph().expect("Can't load graph");

    let graph_inverse_time = Instant::now();
    let mut inversed_graph = Graph::new_directed();
    for node in graph.nodes() {
        inversed_graph.add_node(graph.get_value(node));
    }
    
    for (source, target) in graph.edges() {
        inversed_graph.add_edge(target, source);
    }
    println!("Inversing graph: {:.2?}", graph_inverse_time.elapsed());
    assert!(graph_inverse_time.elapsed().as_millis() < 20);

    let edge_iteration_time = Instant::now();
    let mut i = 0;
    for node in graph.nodes() {
        if graph.get_neighbours(node).len() == 0 && inversed_graph.get_neighbours(node).len() == 0 {
            i += 1;
        }
    }
    println!("Checking neighbours lengths for each node: {:.2?}", edge_iteration_time.elapsed());
    assert!(edge_iteration_time.elapsed().as_millis() < 15);

    println!("Graph size: {}, useless nodes: {}, which is {:.2}%", graph.len(), i, i as f32 / 100.0f32);
    assert_eq!(i, 0, "There should be no useless nodes!");

    assert_eq!(graph.len(), inversed_graph.len(), "Graph and inverse graph are not of the same size!");
}