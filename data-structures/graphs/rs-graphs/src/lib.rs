use std::collections::VecDeque;
use std::collections::HashSet;


#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Node<'a> {
    pub value: char,
    pub adjacent: Vec<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(value: char) -> Node<'a> {
        let adjacent = Vec::new();
        Node {
            value,
            adjacent,
        }
    }

    pub fn add_edge(&mut self, other: &'a Self) -> () {
        self.adjacent.push(other); 
    }
}

#[derive(Debug, Clone)]
pub struct Graph<'a> {
    pub nodes: HashSet<&'a Node<'a>>,
}

impl<'a> Graph<'a> {
    pub fn add_vertex(&mut self, vertex: &'a Node) {
        self.nodes.insert(&vertex);
    }

    pub fn new() -> Graph<'a> {
        let nodes: HashSet<&Node> = HashSet::new();
        Graph {
            nodes,
        }
    }

    pub fn contains(&self, vertex: &'a Node) -> bool {
        self.nodes.contains(&vertex)
    }

    // Designed to consume vertex_array as I can't imagine why you would need
    // to keep it in scope - our entire graph system is built around hashsets
    pub fn add_vertices(&mut self, vertex_array: [&'a Node; 2]) {
        for vertex in vertex_array.iter() {
            self.nodes.insert(&vertex);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vertex() {
        let mut graph = Graph::new(); 
        let a = Node::new('A');
        let b = Node::new('B');
        let c = Node::new('C');
        let d = Node::new('D');

        graph.add_vertex(&c);
        graph.add_vertex(&b);
        
        assert_eq!(graph.contains(&c), true);
        assert_eq!(graph.contains(&b), true);
    }

    #[test]
    fn add_vertices() {
        let mut graph = Graph::new(); 
        let a = Node::new('A');
        let b = Node::new('B');
        let c = Node::new('C');
        let d = Node::new('D');

        let array: [&Node; 2] = [&a, &b];
        graph.add_vertices(array);
        
        assert!(graph.contains(&a));
        assert_eq!(graph.contains(&b), true);
    }
    
    #[test]
    fn add_edge() {
        let mut a: Node = Node::new('A');
        let b: Node = Node::new('B');
        
        a.add_edge(&b);
        // b.add_edge(&a);

        assert_eq!(a.adjacent.contains(&&b), true);
        assert_eq!(b.adjacent.contains(&&a), false);
    }
}
