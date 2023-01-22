use crate::graph::*;

// Note: PriorityNode can use all {integer}s but cannot use {float}s
// due to them implementing neither Ord nor Eq

#[derive (Debug, Eq)]
pub struct PriorityNode<W: Ord> {
    pub priority: W,
    pub node: Node,
}

impl<W: Ord> Ord for PriorityNode<W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<W: Ord> PartialOrd for PriorityNode<W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl<W: Ord> PartialEq for PriorityNode<W> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}


#[test]
fn test_priority_ordering() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    let g = Graph::from([(0,0)]);
    let node = g.nodes().collect::<Vec<Node>>()[0];
    heap.push(PriorityNode{priority:3, node});
    heap.push(PriorityNode{priority:1, node});
    heap.push(PriorityNode{priority:2, node});
    assert_eq!(heap.pop().unwrap().priority, 1);
    assert_eq!(heap.pop().unwrap().priority, 2);
    assert_eq!(heap.pop().unwrap().priority, 3);
}

#[test]
fn test_nodes_dont_matter() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    let mut g = Graph::from([]);
    let node1 = g.add_node(1);
    let node2 = g.add_node(2);
    let node3 = g.add_node(3);
    heap.push(PriorityNode{priority:1,node:node3});
    heap.push(PriorityNode{priority:1,node:node1});
    heap.push(PriorityNode{priority:1,node:node2});

    assert_eq!(heap.pop().unwrap().node, node3);
    assert_eq!(heap.pop().unwrap().node, node1);
    assert_eq!(heap.pop().unwrap().node, node2);
}

#[test]
fn test_ascending_vs_descending_order_push() {
    use std::time::Instant;
    use std::collections::BinaryHeap;
    let g = Graph::from([(1,1)]);
    let node = g.nodes().collect::<Vec<Node>>()[0];
    
    let mut heap = BinaryHeap::new();
    
    let now = Instant::now();
    for i in (0..10000).into_iter() {
        heap.push(PriorityNode{priority:i, node});
    }
    println!("Ascending: {}", now.elapsed().as_micros());

    let now1 = Instant::now();
    for i in (0..10000).into_iter().rev() {
        heap.push(PriorityNode{priority:i, node});
    }
    println!("Descending: {}", now1.elapsed().as_micros());
}