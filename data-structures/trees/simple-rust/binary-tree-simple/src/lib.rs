struct Tree<T> {
    val: T,
    left: TreeNode<T>,
    right: TreeNode<T>,
}

impl<T> Tree<T> {
    fn new(v: T, left: TreeNode<T>, right: TreeNode<T>) -> Tree<T> {
        Tree {
            val: v,
            left,
            right,
        }
    }
}

type TreeNode<T> = Option<Box<Tree<T>>>;

type TreeLevel<'a, T> = Vec<&'a Option<Box<Tree<T>>>>;


/// Simple test to show BFS tree level by tree level approach iterative
///
use std::collections::VecDeque;

fn isTreeSymmetric(t: TreeNode<i32>) -> bool {
    let mut deque = VecDeque::new();

    deque.push_back(vec![&t]);

    while !deque.is_empty() {
        let level = deque.pop_front().unwrap();

        // check if current level is symmetric
        if !is_level_symmetric(&level) {
            return false;
        }

        // push children of all nodes at this level into next_level vec
        let mut next_level = Vec::new();
        for node in level {
            if let Some(n) = node {
                next_level.push(&n.left);
                next_level.push(&n.right);
            }
        }

        if !next_level.is_empty() {
            deque.push_back(next_level);
        }
    }

    true
}

fn is_level_symmetric(level: &TreeLevel<i32>) -> bool {
    for i in 0..level.len() / 2 {
        if let Some(l_node) = level[i] {
            if let Some(r_node) = level[level.len() - 1 - i] {
                if l_node.val != r_node.val {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            if let Some(_) = level[level.len() - 1 - i] {
                return false;
            }
        }
    }

    true
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structure_test() {
        // build up a very basic tree
        let tree = Tree::new(5, Some(Box::new(Tree::new(4, None, None))), Some(Box::new(Tree::new(6, None, None))));

        assert_eq!(tree.left.unwrap().val, 4);
        assert_eq!(tree.right.unwrap().val, 6);
    }
}
