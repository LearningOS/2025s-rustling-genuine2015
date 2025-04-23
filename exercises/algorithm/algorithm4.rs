/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let BinarySearchTree {
            root
        } = self;

        if root.is_none() {
            *root = Some(Box::new(TreeNode::new(value)));
        } else {
            root.as_mut().unwrap().as_mut().insert(value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let BinarySearchTree {
            root
        } = self;

        root.as_ref()
            .map(Box::as_ref)
            .map(|n| n.search(value))
            .unwrap_or(false)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        let Self {
            value: val,
            left,
            right
        } = self;

        let target = match (&value).cmp(val) {
            Ordering::Less => left,
            Ordering::Greater => right,
            Ordering::Equal => return,
        };

        if target.is_none() {
            *target = Some(Box::new(TreeNode::new(value)));
        } else {
            target.as_mut().unwrap().as_mut().insert(value);
        }
    }

    fn search(&self, value: T) -> bool {
        //TODO
        let Self {
            value: val,
            left,
            right
        } = self;

        let target = if &value < val {
            left
        } else if &value == val {
            return true
        } else {
            right
        };

        target
            .as_ref()
            .map(Box::as_ref)
            .map(|n| n.search(value))
            .unwrap_or(false)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


