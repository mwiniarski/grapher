pub struct Connection<EdgeIndex> {
    pub node_index: usize,
    pub edge_index: EdgeIndex
}

pub struct AdjancencyList<EdgeIndex> {
    list: Vec<Vec<Connection<EdgeIndex>>>
}

pub struct NodeIterator<'a, EdgeIndex> {
    list: &'a AdjancencyList<EdgeIndex>,
    node_index: usize
}

pub struct EdgeIterator<'a, EdgeIndex> {
    list: &'a AdjancencyList<EdgeIndex>,
    node_index: usize,
    neighbour_index: usize
}

impl<EdgeIndex> AdjancencyList<EdgeIndex> {
    pub fn new() -> Self {
        AdjancencyList { list: Vec::new() }
    }

    pub fn add_node(&mut self) {
        self.list.push(Vec::new());
    }

    pub fn add_edge(&mut self, source: usize, target: usize, edge_index: EdgeIndex) {
        self.list[source].push(Connection{node_index: target, edge_index: edge_index});
    }

    pub fn get_neighbours(&self, node: usize) -> &Vec<Connection<EdgeIndex>> {
        &self.list[node]
    }

    pub fn node_exists(&self, node: usize) -> bool {
        node < self.list.len()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn nodes(&self) -> NodeIterator<EdgeIndex> {
        NodeIterator { list: &self, node_index: 0 }
    }

    pub fn edges(&self) -> EdgeIterator<EdgeIndex> {
        EdgeIterator { list: &self, node_index: 0, neighbour_index: 0 }
    }
}


impl<'a, EdgeIndex> Iterator for NodeIterator<'a, EdgeIndex> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.list.node_exists(self.node_index) {
            return None
        }

        let ret = Some(self.node_index);
        self.node_index += 1;
        ret
    }
}

impl<EdgeIndex> Iterator for EdgeIterator<'_, EdgeIndex> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.find_next_existing_edge() {
            return None;
        }

        let ret = Some((
            self.node_index,
            self.list.get_neighbours(self.node_index)[self.neighbour_index].node_index
        ));

        self.neighbour_index += 1;
        ret
    }
}

impl<EdgeIndex> EdgeIterator<'_, EdgeIndex> {
    fn find_next_existing_edge(&mut self) -> bool {
        while self.list.node_exists(self.node_index) {
            if self.neighbour_index < self.list.get_neighbours(self.node_index).len() {
                return true;
            }
            self.node_index += 1;
            self.neighbour_index = 0;
        }
        false
    }
}