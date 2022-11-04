#[derive(PartialEq, Eq, Debug)]
pub struct PriorityNode {
    pub priority: usize,
    pub node: usize,
}

impl Ord for PriorityNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for PriorityNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

#[test]
fn test_priority_ordering() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push(PriorityNode{priority:3,node:2});
    heap.push(PriorityNode{priority:1,node:2});
    heap.push(PriorityNode{priority:2,node:2});
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
    //let vec:Vec<usize> = (0..10000).collect();
    
    let now = Instant::now();
    for i in (0..10000).into_iter() {
        heap.push(PriorityNode{priority:i, node:1});
    }
    println!("Ascending: {}", now.elapsed().as_micros());

    let now1 = Instant::now();
    for i in (0..10000).into_iter().rev() {
        heap.push(PriorityNode{priority:i, node:1});
    }
    println!("Descending: {}", now1.elapsed().as_micros());
}

// for _ in 0..heap.len() {
//     println!("POP: {:?}", heap.pop());
//}