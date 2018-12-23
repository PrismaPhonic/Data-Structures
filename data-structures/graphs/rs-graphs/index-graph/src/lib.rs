#![feature(test)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

extern crate test;

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
    // for right row this is just for unit testing
    pub fn is_edge_of(&self, a: usize, b: usize) -> bool {
        self.nodes.get(&b).unwrap().adjacent.contains(&a)
    }

    // We get the next free index and, push a fresh node with
    // data supplied to our Graph and then return the index
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
        // existed in any of them and remove
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

    
    pub fn shortest_path(&self, start: usize, end: usize) -> i32 {
        let mut queue = VecDeque::new();
        let first = vec![start];
        queue.push_back(first);
        let mut visited = HashSet::new();
        visited.insert(start);
        let mut count = 0;

        loop {
            let curr_group = queue.pop_front().unwrap();

            let mut next_group: Vec<usize> = vec![];
            for curr_index in curr_group {
                if curr_index == end {
                    return count;
                }

                let adjacent = &self.nodes.get(&curr_index).unwrap().adjacent;
                adjacent.iter().for_each(|neighbor| {
                    if !visited.contains(neighbor) {
                        next_group.push(*neighbor);
                        visited.insert(*neighbor);
                    }
                });
            }
            if !next_group.is_empty() {
                queue.push_back(next_group);
            }
            count += 1;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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

    #[test]
    fn shortest_path() {
        let mut graph = Graph::new(); 
        let s = graph.new_node('S');
        let p = graph.new_node('P');
        let u = graph.new_node('U');
        let x = graph.new_node('X');
        let q = graph.new_node('Q');
        let y = graph.new_node('Y');
        let v = graph.new_node('V');
        let r = graph.new_node('R');
        let w = graph.new_node('W');
        let t = graph.new_node('T');
        
        graph.link_nodes(s, p);
        graph.link_nodes(s, u);

        graph.link_nodes(p, x);
        graph.link_nodes(u, x);
        
        graph.link_nodes(p, q);
        graph.link_nodes(u, v);
        
        graph.link_nodes(x, q);
        graph.link_nodes(x, y);
        graph.link_nodes(x, v);

        graph.link_nodes(q, r);
        graph.link_nodes(y, r);
        
        graph.link_nodes(y, w);
        graph.link_nodes(v, w);
        
        graph.link_nodes(r, t);
        graph.link_nodes(w, t);

        assert_eq!(
            graph.shortest_path(p, v),
            2
        );
        assert_eq!(
            graph.shortest_path(p, y),
            2
        );
        assert_eq!(
            graph.shortest_path(p, w),
            3
        );
        assert_eq!(
            graph.shortest_path(s, t),
            4
        );
    }
    
    #[bench]
    fn bench_shortest_path(b: &mut Bencher) {
        let mut graph = Graph::new(); 
        let s = graph.new_node('S');
        let p = graph.new_node('P');
        let u = graph.new_node('U');
        let x = graph.new_node('X');
        let q = graph.new_node('Q');
        let y = graph.new_node('Y');
        let v = graph.new_node('V');
        let r = graph.new_node('R');
        let w = graph.new_node('W');
        let t = graph.new_node('T');
        
        graph.link_nodes(s, p);
        graph.link_nodes(s, u);

        graph.link_nodes(p, x);
        graph.link_nodes(u, x);
        
        graph.link_nodes(p, q);
        graph.link_nodes(u, v);
        
        graph.link_nodes(x, q);
        graph.link_nodes(x, y);
        graph.link_nodes(x, v);

        graph.link_nodes(q, r);
        graph.link_nodes(y, r);
        
        graph.link_nodes(y, w);
        graph.link_nodes(v, w);
        
        graph.link_nodes(r, t);
        graph.link_nodes(w, t);

        b.iter(|| graph.shortest_path(s, t));
    }
}
