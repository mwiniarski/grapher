use crate::graph::Graph;
use crate::priority_node;

pub struct PathFinder;

impl PathFinder {
    
    ///
    /// Finds shortest path between a and b, using Dijkstra algorightm.
    /// Returns None if there is no path between a and b.
    ///
    /// # Panics
    /// Panics if either of nodes does not exist in the graph.
    ///
    pub fn find_shortest_path<T: Graph>(graph: &T, source: usize, target: usize) -> Option<Vec<usize>> {
        assert!(graph.node_exists(source), "Source node doesn't exist");
        assert!(graph.node_exists(target), "Target node doesn't exist");

        let mut distance = vec![std::usize::MAX; graph.size()];
        let mut visited = vec![false; graph.size()];
        let mut previous = vec![Option::<usize>::None; graph.size()];
        let mut queue:std::collections::BinaryHeap<priority_node::PriorityNode> = std::collections::BinaryHeap::new();

        // Initialize state
        queue.push(priority_node::PriorityNode{priority:0, node:source});
        let mut target_reached = false;

        'outer: while !queue.is_empty() {
            
            // Take vertex from queue with lowest distance value
            let curr_vertex = queue.pop().unwrap(); // TODO: match on this instead of while?

            // Check if the vertex was not already handled. Since we are adding new vertices instead of decreasing
            // their priority, duplicates happen.
            if visited[curr_vertex.node] {
                continue;
            }
            visited[curr_vertex.node] = true;
            
            // For every adjecent node
            for neighbour in graph.get_neighbours(curr_vertex.node) {
                // Check if distance to current node + distance to that neighbour is lower than its saved distance from source
                let dist_through_curr_vertex = curr_vertex.priority + 1 /* TODO: replace with weight */;
                if  dist_through_curr_vertex < distance[*neighbour] {
                    
                    // If yes then replace that distance and add to queue
                    distance[*neighbour] = dist_through_curr_vertex;
                    previous[*neighbour] = Some(curr_vertex.node);
                    
                    // Finding the target in queue finishes the algorithm
                    if *neighbour == target {
                        target_reached = true;
                        break 'outer;
                    }

                    queue.push(priority_node::PriorityNode{ priority: dist_through_curr_vertex, node: *neighbour });
                }
            }
        }

        // If we haven't found the target, the path doesn't exist
        if !target_reached {
            return None
        }

        // Allocate a zero vector and iterate over it from behind
        let mut ret = vec![0; distance[target] + 1];
        let mut curr_vertex = target;

        for elem in ret.iter_mut().rev() {
            *elem = curr_vertex;

            // If there is a previous, continue to it - there always should be
            match previous[curr_vertex] {
                Some(i) => {  
                    curr_vertex = i;
                },
                None => {
                    assert_eq!(curr_vertex, source);
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
    pub fn find_all_paths<T: Graph>(graph: &T, source: usize, target: usize) -> Vec<Vec<usize>> {
        assert!(graph.node_exists(source), "Source node doesn't exist");
        assert!(graph.node_exists(target), "Target node doesn't exist");

        let mut all_paths: Vec<Vec<usize>> = Vec::new();
        let mut current_path: Vec<usize> = vec![source];
        PathFinder::find_paths(graph, &mut all_paths, &mut current_path, target);
        all_paths
    }

    fn find_paths<T: Graph>(
        graph: &T,
        all_paths: &mut Vec<Vec<usize>>,
        current_path: &mut Vec<usize>,
        target: usize,
    ) {
        let current_node = *current_path.last().unwrap(); // TODO: check this
        for n in graph.get_neighbours(current_node).iter() {
            
            if *n == target {

                let mut cloned_path = current_path.clone();
                cloned_path.push(*n);
                all_paths.push(cloned_path);

            } else {

                // Check if we haven't visited that node already
                // TODO: optimize with color map (bool vector)
                if current_path
                        .iter()
                        .find(|visited_node| n == *visited_node)
                        .is_none() {
                    
                    // Go deeper
                    current_path.push(*n);
                    PathFinder::find_paths(graph, all_paths, current_path, target);
                    current_path.pop();
                }
            }
        }
    }
}