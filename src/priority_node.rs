// Note: PriorityNode can use all {integer}s but cannot use {float}s
// due to them implementing neither Ord nor Eq
use std::hash::Hash;

#[derive (Debug, Eq)]
pub struct PriorityNode<W: Ord, Node> {
    pub priority: W,
    pub node: Node,
}

impl<W: Ord, Node: Eq + Hash> Ord for PriorityNode<W, Node> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<W: Ord, Node> PartialOrd for PriorityNode<W, Node> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl<W: Ord, Node> PartialEq for PriorityNode<W, Node> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}


#[test]
fn test_priority_ordering() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push(PriorityNode{priority:3, node: 'a'});
    heap.push(PriorityNode{priority:1, node: 'a'});
    heap.push(PriorityNode{priority:2, node: 'a'});
    assert_eq!(heap.pop().unwrap().priority, 1);
    assert_eq!(heap.pop().unwrap().priority, 2);
    assert_eq!(heap.pop().unwrap().priority, 3);
}

#[test]
fn test_nodes_dont_matter() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push(PriorityNode{priority:1,node:3});
    heap.push(PriorityNode{priority:1,node:1});
    heap.push(PriorityNode{priority:1,node:2});

    assert_eq!(heap.pop().unwrap().node, 3);
    assert_eq!(heap.pop().unwrap().node, 1);
    assert_eq!(heap.pop().unwrap().node, 2);
}

#[test]
fn test_ascending_vs_descending_order_push() {
    use std::time::Instant;
    use std::collections::BinaryHeap;
  
    let mut heap = BinaryHeap::new();
    
    let now = Instant::now();
    for i in (0..10000).into_iter() {
        heap.push(PriorityNode{priority:i, node: "ff"});
    }
    println!("Ascending: {}", now.elapsed().as_micros());

    let now1 = Instant::now();
    for i in (0..10000).into_iter().rev() {
        heap.push(PriorityNode{priority:i, node: "ff"});
    }
    println!("Descending: {}", now1.elapsed().as_micros());
}