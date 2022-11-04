use std::fmt;

pub trait Graph {
    fn size(&self) -> usize;
    fn add_edge(&mut self, source: usize, target: usize);
    fn node_exists(&self, node: usize) -> bool;
    fn get_neighbours(&self, node: usize) -> &Vec<usize>;
}

/// Graph
///
/// Adjacency list might look like
/// 0 [1,2]
/// 1 [ ]
/// 2 [3]
/// 3 [1]
/// Number of rows is equal to the number of nodes
/// Numbers in each row represent edges from row index to said number
///
///

pub struct DirectedGraph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph for DirectedGraph {
    ///
    /// Adds an edge between nodes a and b. Creates nodes if they don't exist.
    ///
    fn add_edge(&mut self, source: usize, target: usize) {
        // Create nodes
        while !self.node_exists(source) || !self.node_exists(target) {
            self.adjacency_list.push(Vec::new());
        }

        // New edge
        self.adjacency_list[source].push(target);
    }

    fn node_exists(&self, node: usize) -> bool {
        node < self.adjacency_list.len()
    }

    fn size(&self) -> usize {
        self.adjacency_list.len()
    }

    fn get_neighbours(&self, node: usize) -> &Vec<usize> {
        &self.adjacency_list[node]
    }
}

impl DirectedGraph {

    pub fn new() -> Self {
        DirectedGraph {
            adjacency_list: Vec::new(),
        }
    }

    fn print(&self, pretty: bool) -> String {
        let mut s = String::new();
        for (row_index, row) in self.adjacency_list.iter().enumerate() {
            s.push_str(&format!("{}[", row_index));
            for (num_index, num) in row.iter().enumerate() {
                s.push_str(&num.to_string());
                if num_index < row.len() - 1 {
                    s.push(',');
                }
            }
            s.push_str(&format!("]{}", if pretty { "" } else { "\n" }));
        }
        s
    }
}

impl fmt::Display for DirectedGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(false))
    }
}

impl fmt::Debug for DirectedGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(true))
    }
}