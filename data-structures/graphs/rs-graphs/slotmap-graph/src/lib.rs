use std::collections::VecDeque;
use std::collections::HashSet;

extern crate slotmap;
use slotmap::{new_key_type, Key, SlotMap, Slottable};

new_key_type! {
    struct ListKey;
}


#[derive(Copy, Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub adjacent: ListKey,
}

impl<T> Node<T> {
    pub fn new(value: char) -> Node<T> {
        let adjacent = HashSet::new();
        Node {
            value,
            adjacent,
        }
    }

    pub fn add_edge(&mut self, other: ListKey) -> () {
        self.adjacent.insert(other); 
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T> {
    pub nodes: SlotMap<ListKey, Node<T>>,
}

impl<T> Graph<T> {
    pub fn add_vertex(&mut self, vertex: ListKey) {
        self.nodes.insert(vertex);
    }

    pub fn new() -> Graph<T> {
        let nodes: SlotMap<ListKey, Node<T>> = SlotMap::new();
        Graph {
            nodes,
        }
    }

    pub fn contains(&self, vertex: ListKey) -> bool {
        self.nodes.contains_key(ListKey)
    }

    // Designed to consume vertex_array as I can't imagine why you would need
    // to keep it in scope - our entire graph system is built around hashsets
    pub fn add_vertices(&mut self, vertex_array: [ListKey; 2]) {
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
