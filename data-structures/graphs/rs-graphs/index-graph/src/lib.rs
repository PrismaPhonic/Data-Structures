use std::collections::HashMap;

/**
 * Non-directional graph representation with a memory arena
 * and adjacency list
*/

// Here we use a central storage location for all our nodes
// Implementations that act to create nodes can operate soley
// within the space of the memory arena
#[derive(Debug, Clone)]
struct Graph<T> {
    nodes: HashMap<usize, Node<T>>,
    next_index: usize
}

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    adjacent: Vec<usize>,
}

// We get the next free index and, push a fresh node with
// data supplied to our Graph and then return the index
impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        let nodes: HashMap<usize, Node<T>> = HashMap::new();
        Graph {
            nodes,
            next_index: 0
        }
    }
    
    // bi-directional - only works if both nodes exist - FIX THIS
    pub fn is_connected(&self, n1: usize, n2: usize) -> bool {
        self.nodes.get(&n1).unwrap().adjacent.contains(&n2) && self.nodes.get(&n2).unwrap().adjacent.contains(&n1)
    }

    // is a an edge of b? this is a one directional search only and works
    // if the edge we are checking for is a node that has been deleted
    pub fn is_edge_of(&self, a: usize, b: usize) -> bool {
        self.nodes.get(&b).unwrap().adjacent.contains(&a)
    }

    pub fn new_node(&mut self, data: T) -> usize {
        let index = self.next_index;

        self.nodes.insert(index, Node {
            data,
            adjacent: Vec::new(),
        });

        self.next_index += 1;
        index
    }

    pub fn remove_node(&mut self, n: usize) -> bool {
        if let None = self.nodes.remove(&n) {
            return false
        }

        // search through all nodes adjaceny lists to see if index
        // existed inn any of them
        for (_k, v) in self.nodes.iter_mut() {
            if v.adjacent.contains(&n) {
                v.adjacent.remove(n);
            }
        }

        true
    }

    // links two nodes adding an edge to each nodes adjacency list
    // return success or failure
    pub fn link_nodes(&mut self, n1: usize, n2: usize) -> Result<(), &'static str> {
        let max_index = self.nodes.len()-1;
        if n1 > max_index || n2 > max_index {
            return Err("Node indices are too high!");
        }
    
        self.nodes.get_mut(&n1).unwrap().adjacent.push(n2);
        self.nodes.get_mut(&n2).unwrap().adjacent.push(n1);

        Ok(())
    }

    pub fn contains(&self, n: usize) -> bool {
        if let None = self.nodes.get(&n) {
            return false;
        } else {
            return true;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vertex() {
        let mut graph = Graph::new(); 
        let a = graph.new_node('A');
        let b = graph.new_node('B');
        let c = graph.new_node('C');
        let d = graph.new_node('D');

        assert_eq!(graph.contains(c), true);
        assert_eq!(graph.contains(b), true);
    }

    #[test]
    fn link_nodes() {
        let mut graph = Graph::new(); 
        let a = graph.new_node('A');
        let b = graph.new_node('B');
        let c = graph.new_node('C');
        let d = graph.new_node('D');

        graph.link_nodes(a, b);

        assert_eq!(graph.is_connected(a, b), true);
        assert_eq!(graph.is_connected(a, c), false);
    }

    #[test]
    fn remove_node() {
        let mut graph = Graph::new(); 
        let a = graph.new_node('A');
        let b = graph.new_node('B');
        let c = graph.new_node('C');
        let d = graph.new_node('D');
        
        graph.link_nodes(a, b);

        assert_eq!(graph.contains(a), true);
        assert_eq!(graph.is_connected(a, b), true);

        graph.remove_node(a);

        assert_eq!(graph.contains(a), false);
        assert_eq!(graph.is_edge_of(a, b), false);
    }
}
