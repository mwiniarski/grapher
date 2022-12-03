
pub struct AdjancencyList {
    list: Vec<Vec<usize>>
}

impl AdjancencyList {
    pub fn new() -> Self {
        AdjancencyList { list: Vec::new() }
    }

    pub fn add_node(&mut self) {
        self.list.push(Vec::new());
    }

    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.list[source].push(target);
    }

    pub fn get_neighbours(&self, node: usize) -> &Vec<usize> {
        &self.list[node]
    }

    pub fn node_exists(&self, node: usize) -> bool {
        node < self.list.len()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn nodes(&self) -> NodeIterator {
        NodeIterator { list: &self, current_node_index: 0 }
    }

    pub fn edges(&self) -> EdgeIterator {
        EdgeIterator { list: &self, current_node_index: 0, current_neighbour_index: 0 }
    }
}

pub struct NodeIterator<'a> {
    list: &'a AdjancencyList,
    current_node_index: usize
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.list.node_exists(self.current_node_index) {
            return None
        }

        let ret = Some(self.current_node_index);
        self.current_node_index += 1;
        ret
    }
}

pub struct EdgeIterator<'a> {
    list: &'a AdjancencyList,
    current_node_index: usize,
    current_neighbour_index: usize
}

impl Iterator for EdgeIterator<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.find_next_existing_edge() {
            return None;
        }

        let ret = Some((
            self.current_node_index,
            self.list.get_neighbours(self.current_node_index)[self.current_neighbour_index]
        ));

        self.current_neighbour_index += 1;
        ret
    }
}

impl EdgeIterator<'_> {
    fn find_next_existing_edge(&mut self) -> bool {
        while self.list.node_exists(self.current_node_index) {
            if self.current_neighbour_index < self.list.get_neighbours(self.current_node_index).len() {
                return true;
            }
            self.current_node_index += 1;
            self.current_neighbour_index = 0;
        }
        false
    }
}