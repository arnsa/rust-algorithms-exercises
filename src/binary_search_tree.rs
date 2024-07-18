/// Binary search tree implementation
///
/// # Example
/// ```
/// let binary_search_tree = algorithms_exercises::binary_search_tree::BinarySearchTree::new(Vec::from([1, 3, 2, 5, 4]));
/// assert_eq!(binary_search_tree.root.as_ref().unwrap().value, 1);
/// assert_eq!(binary_search_tree.find(&3), true);
/// ```
use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone + Debug> Node<T> {
    fn new(value: T) -> Self {
        return Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    pub root: Option<Box<Node<T>>>
}

impl<T: Ord + Clone + Debug> BinarySearchTree<T> {
    pub fn new(values: Vec<T>) -> Self {
        let mut result = Self {
            root: None
        };

        for value in values {
            result.add(value);
        }

        return result;
    }

    pub fn add(&mut self, value: T) {
        let mut current = &mut self.root;

        while let Some(ref mut node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => { current = &mut node.left; }
                Ordering::Greater => { current = &mut node.right; }
                Ordering::Equal => return
            }
        }

        *current = Some(Box::new(Node::new(value)));
    }

    pub fn find(&self, value: &T) -> bool {
        let mut current = &self.root;

        while let Some(ref node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => { current = &node.left; }
                Ordering::Greater => { current = &node.right; }
                Ordering::Equal => return true
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_tree() {
        let binary_search_tree = BinarySearchTree::new(Vec::from([3, 7, 4, 6, 5, 1, 10, 2, 9, 8]));

        assert_eq!(binary_search_tree.root.as_ref().unwrap().value, 3);

        assert_eq!(binary_search_tree.root.as_ref().unwrap().left.as_ref().unwrap().value, 1);
        assert!(binary_search_tree.root.as_ref().unwrap().left.as_ref().unwrap().left.is_none());

        assert_eq!(binary_search_tree.root.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().value, 2);
        assert!(binary_search_tree.root.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().left.is_none());
        assert!(binary_search_tree.root.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().right.is_none());

        assert_eq!(binary_search_tree.root.as_ref().unwrap().right.as_ref().unwrap().value, 7);
        assert_eq!(binary_search_tree.root.as_ref().unwrap().right.as_ref().unwrap().left.as_ref().unwrap().value, 4);
        assert!(binary_search_tree.root.as_ref().unwrap().right.as_ref().unwrap().left.as_ref().unwrap().left.is_none());

        assert_eq!(binary_search_tree.root.as_ref().unwrap().right.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().value, 6);
        assert_eq!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap().value,
            5
        );
        assert!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().is_none()
        );
        assert!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().is_none()
        );

        assert_eq!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .value,
            10
        );
        assert!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().is_none()
        );

        assert_eq!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .value,
            9
        );
        assert!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().is_none()
        );

        assert_eq!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().unwrap()
                .value,
            8
        );
        assert!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().is_none()
        );
        assert!(
            binary_search_tree
                .root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().is_none()
        );

        assert_eq!(binary_search_tree.find(&3), true);
        assert_eq!(binary_search_tree.find(&10), true);
        assert_eq!(binary_search_tree.find(&11), false);
    }
}
