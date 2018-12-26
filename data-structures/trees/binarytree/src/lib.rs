#![feature(test)]
use std::collections::VecDeque;
use std::collections::HashMap;

extern crate test;

struct Tree<T> {
    nodes: HashMap<usize, Node<T>>,
    next_index: usize,
    root: usize,
}

#[derive(Debug)]
#[derive(Clone)]
struct Node<T> {
    data: T,
    children: Vec<usize>,
}

impl<T> Node<T> {
    pub fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        let nodes: HashMap<usize, Node<T>> = HashMap::new();
        Tree {
            nodes,
            next_index: 0,
            root: 0,
        }
    }

    pub fn new_node(&mut self, data: T) -> usize {
        let index = self.next_index;

        self.nodes.insert(index, Node {
            data,
            children: Vec::new(),
        });

        self.next_index += 1;
        index
    }

    pub fn set_root(&mut self, root: usize) {
        self.root = root;
    }

    pub fn add_child_to_node(&mut self, node: usize, child: usize) {
        match self.nodes.get_mut(&node) {
            Some(node) => node.children.push(child),
            None => (),
        }
    }

    pub fn find(&self, index: usize) -> Result<&Node<T>, &'static str> {
       match self.nodes.get(&index) {
           Some(node) => return Ok(node),
           None => return Err("No node at that index")
       }
    }

    pub fn contains(&self, index: usize) -> bool {
        match self.nodes.get(&index) {
            Some(_) => return true,
            None => return false,
        }
    }

    pub fn min_depth(&self) -> i32 {
        //count depth until we reach the first leaf
        let root: Vec<usize> = vec![self.root];
        let mut to_visit_queue = VecDeque::new();
        to_visit_queue.push_back(root);
        let mut depth = 0;

        loop {
            let current_rank_deque = to_visit_queue.pop_front().unwrap();
            let mut next_children = Vec::new();
    
            for neighbor in current_rank_deque {
                let neighbor_children = self.nodes[&neighbor].children.clone();
                if neighbor_children.is_empty() {
                    return depth
                } else {
                    neighbor_children.into_iter().for_each(|c| {
                        next_children.push(c);
                    })
                }
            }

            to_visit_queue.push_back(next_children);
            depth += 1;
        }
    }

    // for now you must supply the root node index when calling
    // because rust does not have default parameter values as of now
    pub fn max_depth_recursive(&self) -> i32 {
        fn recurse_depth<T>(tree: &Tree<T>, node: usize, depth: i32, mut max: i32) -> i32 {
            if tree.nodes[&node].children.is_empty() { return depth };

            tree.nodes[&node].children.iter().for_each(|child| {
                let max_from_branch = recurse_depth(tree, *child, depth + 1, max);
                if max_from_branch > max { max = max_from_branch };
            });
            
            max
        }
       recurse_depth(self, self.root, 0, 0)
    }

    pub fn max_depth_iter(&self) -> usize {
        let mut depth = 0;
        let root = Current::new(self.root, depth);
        let mut to_visit_stack: Vec<Current> = vec![root];
        let mut max = 0;

        while !to_visit_stack.is_empty() {
            let current = to_visit_stack.pop().unwrap();
            depth = current.depth;
            if depth > max { max = depth }
            self.nodes[&current.node].children.iter().for_each(|c| {
               to_visit_stack.push(Current::new(*c, depth + 1)); 
            });
        }

        max
    }
}

// created specifically for testing iter vs recursive solutions for max
// depth functions - results - same efficiency in benchmarking
struct Current {
    depth: usize,
    node: usize,
}

impl Current {
    fn new(node: usize, depth: usize) -> Current {
        Current {
            depth,
            node,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn new_node() {
        let mut tree = Tree::new(); 
        let one = tree.new_node(1);
        let two = tree.new_node(2);
        let three = tree.new_node(3);
        let four = tree.new_node(4);
        let five = tree.new_node(5);
        let six = tree.new_node(6);
        let seven = tree.new_node(7);
        let eight = tree.new_node(8);

        assert_eq!(tree.contains(one), true);
        assert_eq!(tree.contains(seven), true);
    }

    #[test]
    fn add_child() {
        let mut tree = Tree::new(); 
        let one = tree.new_node(1);
        let two = tree.new_node(2);
        let three = tree.new_node(3);
        let four = tree.new_node(4);
        let five = tree.new_node(5);
        let six = tree.new_node(6);
        let seven = tree.new_node(7);
        let eight = tree.new_node(8);

        tree.add_child_to_node(one, two);
        tree.add_child_to_node(one, three);
        tree.add_child_to_node(one, four);

        tree.add_child_to_node(two, five);
        tree.add_child_to_node(two, six);

        tree.add_child_to_node(three, seven);
        tree.add_child_to_node(three, eight);

        let one_node = tree.find(one).unwrap();
        let two_node = tree.find(two).unwrap();

        assert_eq!(one_node.children, [two, three, four]);
        assert_eq!(two_node.children, [five, six]);
    }

    #[test]
    fn min_depth() {
        let mut tree = Tree::new(); 
        let one = tree.new_node(1);
        let two = tree.new_node(2);
        let three = tree.new_node(3);
        let four = tree.new_node(4);
        let five = tree.new_node(5);
        let six = tree.new_node(6);
        let seven = tree.new_node(7);
        let eight = tree.new_node(8);

        tree.add_child_to_node(one, two);
        tree.add_child_to_node(one, three);
        tree.add_child_to_node(one, four);

        tree.add_child_to_node(two, five);
        tree.add_child_to_node(two, six);

        tree.add_child_to_node(three, seven);
        tree.add_child_to_node(three, eight);

        assert_eq!(tree.min_depth(), 1);
    }

    #[test]
    fn max_depth() {
        let mut tree = Tree::new(); 
        let one = tree.new_node(1);
        let two = tree.new_node(2);
        let three = tree.new_node(3);
        let four = tree.new_node(4);
        let five = tree.new_node(5);
        let six = tree.new_node(6);
        let seven = tree.new_node(7);
        let eight = tree.new_node(8);

        tree.add_child_to_node(one, two);
        tree.add_child_to_node(one, three);
        tree.add_child_to_node(one, four);

        tree.add_child_to_node(two, five);
        tree.add_child_to_node(two, six);

        tree.add_child_to_node(three, seven);
        tree.add_child_to_node(three, eight);

        assert_eq!(tree.max_depth_recursive(), 2);
    }

    #[test]
    fn max_depth_iter() {
        let mut tree = Tree::new(); 
        let one = tree.new_node(1);
        let two = tree.new_node(2);
        let three = tree.new_node(3);
        let four = tree.new_node(4);
        let five = tree.new_node(5);
        let six = tree.new_node(6);
        let seven = tree.new_node(7);
        let eight = tree.new_node(8);

        tree.add_child_to_node(one, two);
        tree.add_child_to_node(one, three);
        tree.add_child_to_node(one, four);

        tree.add_child_to_node(two, five);
        tree.add_child_to_node(two, six);

        tree.add_child_to_node(three, seven);
        tree.add_child_to_node(three, eight);

        assert_eq!(tree.max_depth_iter(), 2);
    }

    #[bench]
    fn max_depth_recursive_bench(b: &mut Bencher) {
        let mut tree = Tree::new(); 
        let one = tree.new_node(1);
        let two = tree.new_node(2);
        let three = tree.new_node(3);
        let four = tree.new_node(4);
        let five = tree.new_node(5);
        let six = tree.new_node(6);
        let seven = tree.new_node(7);
        let eight = tree.new_node(8);
        let nine = tree.new_node(9);
        let ten = tree.new_node(10);
        let eleven = tree.new_node(11);
        let twelve = tree.new_node(12);

        tree.add_child_to_node(one, two);
        tree.add_child_to_node(one, three);
        tree.add_child_to_node(one, four);

        tree.add_child_to_node(two, five);
        tree.add_child_to_node(two, six);

        tree.add_child_to_node(three, seven);
        tree.add_child_to_node(three, eight);

        tree.add_child_to_node(five, nine);
        tree.add_child_to_node(five, ten);
        
        tree.add_child_to_node(ten, eleven);
        tree.add_child_to_node(ten, twelve);
        
        b.iter(|| tree.max_depth_recursive());
    }

    #[bench]
    fn max_depth_iter_bench(b: &mut Bencher) {
        let mut tree = Tree::new(); 
        let one = tree.new_node(1);
        let two = tree.new_node(2);
        let three = tree.new_node(3);
        let four = tree.new_node(4);
        let five = tree.new_node(5);
        let six = tree.new_node(6);
        let seven = tree.new_node(7);
        let eight = tree.new_node(8);
        let nine = tree.new_node(9);
        let ten = tree.new_node(10);
        let eleven = tree.new_node(11);
        let twelve = tree.new_node(12);

        tree.add_child_to_node(one, two);
        tree.add_child_to_node(one, three);
        tree.add_child_to_node(one, four);

        tree.add_child_to_node(two, five);
        tree.add_child_to_node(two, six);

        tree.add_child_to_node(three, seven);
        tree.add_child_to_node(three, eight);

        tree.add_child_to_node(five, nine);
        tree.add_child_to_node(five, ten);
        
        tree.add_child_to_node(ten, eleven);
        tree.add_child_to_node(ten, twelve);
        
        b.iter(|| tree.max_depth_iter());
    }
    
}
