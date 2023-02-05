use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Instant;

use grapher::graph::*;

fn assert_time(time: &Instant, name: &str, limit_ms: f64) {

    let limit_print = if limit_ms < 1.0 { limit_ms * 1000.0 } else { limit_ms };
    let time_unit = if limit_ms < 1.0 { "µs" } else { "ms" };
    println!("{}: {:.2?} / {}{}", name, time.elapsed(), limit_print , time_unit);
    assert!(time.elapsed().as_micros() < (limit_ms * 1000.0f64) as u128);
}

fn load_graph() -> std::io::Result<Graph<usize>> {
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
    let g = Graph::<usize>::from_vec_directed(temp_vec);
    
    assert_time(&time, "Graph::from_vec", 80.0);
    Ok(g)
}

#[test]
fn performance_test1() {
    let graph = load_graph().expect("Can't load graph");

    let graph_inverse_time = Instant::now();
    let mut inversed_graph = Graph::new_directed();
    for node in graph.nodes() {
        inversed_graph.add_node(*node.1);
    }
    
    for edge in graph.edges() {
        inversed_graph.add_edge(edge.target, edge.source);
    }
    assert_time(&graph_inverse_time, "Inversing graph", 27.0);

    let edge_iteration_time = Instant::now();
    let mut i = 0;
    for node in graph.nodes() {
        if graph.get_degree(node.0) == 0 && inversed_graph.get_degree(node.0) == 0 {
            i += 1;
        }
    }
    assert_time(&edge_iteration_time, "Checking neighbours lengths for each node", 0.5);

    println!("Graph size: {}, useless nodes: {}, which is {:.2}%", graph.len(), i, i as f32 / 100.0f32);
    assert_eq!(i, 0, "There should be no useless nodes!");

    assert_eq!(graph.len(), inversed_graph.len(), "Graph and inverse graph are not of the same size!");
}