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

type TreeLevel<T> = Vec<Option<Box<Tree<T>>>>;

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
