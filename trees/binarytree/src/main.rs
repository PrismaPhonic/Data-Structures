use std::collections::VecDeque;

#[derive(Debug)]
#[derive(Clone)]
struct Node {
    val: i32,
    children: Vec<Node>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node { 
            val, 
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

}

#[derive(Debug)]
struct Tree {
    root: Node,
}

impl Tree {
    fn new(root: Node) -> Tree {
        Tree {
            root,
        }
    }

    fn find(&self, val: i32) -> Option<Node> {
        let mut to_visit_stack: Vec<Node> = vec![self.root.clone()];
        
        while !to_visit_stack.is_empty() {
            let current: Node = to_visit_stack.pop().unwrap();
            if current.val != val {
                for child in current.children {
                    to_visit_stack.push(child);
                }
            } else {
                return Some(current.clone());
            }
        }
        None
    }

    fn min_depth(&self) -> i32 {
        //count depth until we reach the first leaf
        let root = vec![self.root.clone()];
        let mut to_visit_queue = VecDeque::new();
        to_visit_queue.push_back(root);
        let mut depth = 0;

        while !to_visit_queue.is_empty() {
            let current_rank_deque = to_visit_queue.pop_front().unwrap();
            let mut next_children = Vec::new();
    
            for neighbor in current_rank_deque {
                let mut neighbor_children = neighbor.children.clone();
                if neighbor_children.is_empty() {
                    return depth
                } else {
                    next_children.append(&mut neighbor_children)
                }
            }

            if !next_children.is_empty() {
                to_visit_queue.push_back(next_children);
            }

            depth += 1;
        }
        depth
    }
}


fn main() {
    let mut root_node = Node::new(1);

    let mut child2 = Node::new(2);
    child2.add_child(Node::new(5));
    child2.add_child(Node::new(6));

    let mut child3 = Node::new(3);
    child3.add_child(Node::new(7));
    child3.add_child(Node::new(8));

    root_node.add_child(child2);
    root_node.add_child(child3);
    root_node.add_child(Node::new(4));


    let tree = Tree::new(root_node);

    println!("Here is your tree: {:#?}", tree);

    println!("Looking for node with value of 3: {:#?}", &tree.find(3).expect("Could not find node with that value!"));

    println!("Minimum Node Depth: {}", &tree.min_depth());
}
