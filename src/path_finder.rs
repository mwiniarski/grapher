use std::collections::HashMap;
use num_traits;

use crate::graph::{Node, NodeIterator};

use crate::priority_node;

pub struct PathFinder;

pub trait PathFindable<'a, W> {
    fn nodes(&'a self) -> NodeIterator<'a>;

    // Iterator over neighbours of the node that returns a tuple of neighbouring node and 
    // weight between current node and it's neighbour
    fn get_neighbours(&'a self, n: Node) -> Box<dyn Iterator<Item=(Node, W)> + 'a>;
}

impl PathFinder {

    ///
    /// Finds shortest path between a and b, using Dijkstra algorightm.
    /// Returns None if there is no path between a and b.
    ///
    /// # Panics
    /// Panics if either of nodes does not exist in the graph.
    ///
    pub fn find_shortest_path<'a, G: PathFindable<'a, Dist>, Dist>(graph: &'a G, source: Node, target: Node) -> Option<Vec<Node>> 
        where Dist: num_traits::PrimInt {

        // Initialize state
        let mut distance: HashMap<Node, Dist> = HashMap::new();
        let mut visited: HashMap<Node, bool> = HashMap::new();
        let mut previous: HashMap<Node, Option<Node>> = HashMap::new();
        let mut queue:std::collections::BinaryHeap<priority_node::PriorityNode<Dist>> = std::collections::BinaryHeap::new();

        queue.push(priority_node::PriorityNode{priority: Dist::zero(), node: source});
        let mut target_reached = false;
        for node in graph.nodes() {
            distance.insert(node, Dist::max_value());
            previous.insert(node, None);
        }

        'outer: while !queue.is_empty() {

            // Take vertex from queue with lowest distance value
            let curr_vertex = queue.pop().unwrap(); // TODO: match on this instead of while?

            // Check if the vertex was not already handled. Since we are adding new vertices instead of decreasing
            // their priority, duplicates happen.
            if visited.contains_key(&curr_vertex.node) {
                continue;
            }
            visited.insert(curr_vertex.node, true);

            // For every adjecent node
            for neighbour in graph.get_neighbours(curr_vertex.node) {
                // Check if distance to current node + distance to that neighbour is lower than its saved distance from source
                let dist_through_curr_vertex = curr_vertex.priority + neighbour.1;
                if  dist_through_curr_vertex < distance[&neighbour.0] {

                    // If yes then replace that distance and add to queue
                    distance.insert(neighbour.0, dist_through_curr_vertex);
                    previous.insert(neighbour.0, Some(curr_vertex.node));

                    // Finding the target in queue finishes the algorithm
                    if neighbour.0 == target {
                        target_reached = true;
                        break 'outer;
                    }

                    queue.push(priority_node::PriorityNode{ priority: dist_through_curr_vertex, node: neighbour.0 });
                }
            }
        }

        // If we haven't found the target, the path doesn't exist
        if !target_reached {
            return None
        }

        // Count the size of return vec
        let mut curr_node = target;
        let mut count = 0;
        loop {
            // If there is a previous, continue to it - there always should be
            match previous[&curr_node] {
                Some(i) => {
                    curr_node = i;
                    count += 1;
                },
                None => {
                    assert_eq!(curr_node, source);
                }
            }
            if curr_node == source {
                break;
            }
        }

        // Allocate a zero vector and iterate over it from behind
        let mut ret = vec![Node::new(); count + 1];

        // let mut ret = vec![0; distance[&target] + 1];
        curr_node = target;

        for elem in ret.iter_mut().rev() {
            *elem = curr_node;

            // If there is a previous, continue to it - there always should be
            match previous[&curr_node] {
                Some(i) => {
                    curr_node = i;
                },
                None => {
                    assert_eq!(curr_node, source);
                }
            }
        }

        Some(ret)

    }

    ///
    /// Finds all paths between a and b
    ///
    /// # Panics
    /// Panics if either of nodes does not exist in the graph or
    /// they are the same node.
    ///
    pub fn find_all_paths<'a, G: PathFindable<'a, Dist>, Dist>(graph: &'a G, source: Node, target: Node) -> Vec<Vec<Node>> {
        let mut all_paths: Vec<Vec<Node>> = Vec::new();
        let mut current_path: Vec<Node> = vec![source];
        PathFinder::find_paths(graph, &mut all_paths, &mut current_path, target);
        all_paths
    }

    fn find_paths<'a, G: PathFindable<'a, Dist>, Dist>(
        graph: &'a G,
        all_paths: &mut Vec<Vec<Node>>,
        current_path: &mut Vec<Node>,
        target: Node,
    ) {
        let current_node = *current_path.last().unwrap(); // TODO: check this
        for n in graph.get_neighbours(current_node) {

            if n.0 == target {

                let mut cloned_path = current_path.clone();
                cloned_path.push(n.0);
                all_paths.push(cloned_path);

            } else {

                // Check if we haven't visited that node already
                // TODO: optimize with color map (bool vector)
                if current_path
                        .iter()
                        .find(|visited_node| n.0 == **visited_node)
                        .is_none() {

                    // Go deeper
                    current_path.push(n.0);
                    PathFinder::find_paths(graph, all_paths, current_path, target);
                    current_path.pop();
                }
            }
        }
    }
}