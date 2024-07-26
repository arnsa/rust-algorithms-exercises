use std::cmp::{max, Ordering};

/// AVL tree implementation
///
/// # Example
/// ```
/// let avl_tree = algorithms_exercises::avl_tree::AVLTree::new(Vec::from([1, 3, 2, 5, 4]));
/// assert_eq!(avl_tree.root.as_ref().unwrap().value, 2);
/// ```

pub struct Node<T> {
    pub value: T,
    height: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> Node<T> {
    fn new(value: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Self {
        return Self {
            value,
            height: 1,
            left,
            right,
        };
    }

    fn add(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                match self.left {
                    Some(ref mut left) => left.add(value),
                    None => self.left = Some(Box::new(Node::new(value, None, None))),
                }
            }
            Ordering::Greater | Ordering::Equal => {
                match self.right {
                    Some(ref mut right) => right.add(value),
                    None => self.right = Some(Box::new(Node::new(value, None, None))),
                }
            }
        }

        self.update_height();
        self.balance();
    }

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |x| x.height);
        let right_height = self.right.as_ref().map_or(0, |x| x.height);

        self.height = max(left_height, right_height) + 1;
    }

    fn balance(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |x| x.height as i32);
        let right_height = self.right.as_ref().map_or(0, |x| x.height as i32);
        let balance_factor = left_height - right_height;

        if balance_factor > 1 {
            let left_left_height = self.left.as_ref().map_or(0, |x| x.left.as_ref().map_or(0, |y| y.height));
            let left_right_height = self.left.as_ref().map_or(0, |x| x.right.as_ref().map_or(0, |y| y.height));

            if left_right_height > left_left_height {
                self.left.as_mut().unwrap().rotate_rr();
            }

            self.rotate_ll();
        } else if balance_factor < -1 {
            let right_right_height = self.right.as_ref().map_or(0, |x| x.right.as_ref().map_or(0, |y| y.height));
            let right_left_height = self.right.as_ref().map_or(0, |x| x.left.as_ref().map_or(0, |y| y.height));

            if right_left_height > right_right_height {
                self.right.as_mut().unwrap().rotate_ll();
            }

            self.rotate_rr();
        }
    }

    fn rotate_rr(&mut self) {
        let prev_left = self.left.take();
        let prev_value = self.value.clone();

        if let Some(right) = self.right.as_deref() {
            self.value = right.value.clone();
            self.left = self.right.take();
        }

        if let Some(ref mut left) = self.left {
            self.right = left.right.take();
            left.right = left.left.take();
            left.left = prev_left;
            left.value = prev_value;

            left.update_in_new_location();
        }

        self.update_in_new_location();
    }

    fn rotate_ll(&mut self) {
        let prev_right = self.right.take();
        let prev_value = self.value.clone();

        if let Some(left) = self.left.as_deref() {
            self.value = left.value.clone();
            self.right = self.left.take();
        }

        if let Some(ref mut right) = self.right {
            self.left = right.left.take();
            right.left = right.right.take();
            right.right = prev_right;
            right.value = prev_value;

            right.update_in_new_location();
        }

        self.update_in_new_location();
    }

    fn update_in_new_location(&mut self) {
        self.height = match (&self.left, &self.right) {
            (None, None) => 1,
            (Some(left), None) => left.height + 1,
            (None, Some(right)) => right.height + 1,
            (Some(left), Some(right)) => std::cmp::max(left.height, right.height) + 1,
        };
    }
}

pub struct AVLTree<T> {
    pub root: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> AVLTree<T> {
    pub fn new(values: Vec<T>) -> Self {
        let mut tree = Self { root: None };

        for value in values {
            tree.add(value);
        }

        return tree;
    }

    pub fn add(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.add(value),
            None => self.root = Some(Box::new(Node::new(value, None, None)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_update_in_new_location() {
        {
            let mut node = Node::new(10, None, None);
            node.update_in_new_location();
            assert_eq!(node.height, 1);
        }

        {
            let mut left_child = Node::new(5, None, None);
            left_child.height = 2;
            let mut node = Node::new(10, Some(Box::new(left_child)), None);
            node.update_in_new_location();
            assert_eq!(node.height, 3);
        }

        {
            let mut right_child = Node::new(15, None, None);
            right_child.height = 2;
            let mut node = Node::new(10, None, Some(Box::new(right_child)));
            node.update_in_new_location();
            assert_eq!(node.height, 3);
        }

        {
            let mut left_child = Node::new(5, None, None);
            left_child.height = 3;
            let mut right_child = Node::new(15, None, None);
            right_child.height = 2;
            let mut node = Node::new(10, Some(Box::new(left_child)), Some(Box::new(right_child)));
            node.update_in_new_location();
            assert_eq!(node.height, 4);
        }

        {
            let mut left_child = Node::new(5, None, None);
            left_child.height = 2;
            let mut right_child = Node::new(15, None, None);
            right_child.height = 3;
            let mut node = Node::new(10, Some(Box::new(left_child)), Some(Box::new(right_child)));
            node.update_in_new_location();
            assert_eq!(node.height, 4);
        }
    }

    #[test]
    fn test_rotate_ll() {
        {
            let right_grand_child = Some(Box::new(Node::new(25, None, None)));
            let left_grand_child = Some(Box::new(Node::new(5, None, None)));
            let left_child = Some(Box::new(Node::new(10, left_grand_child, Some(Box::new(Node::new(15, None, None))))));
            let mut root = Node::new(20, left_child, right_grand_child);

            root.rotate_ll();

            assert_eq!(root.value, 10);
            assert_eq!(root.left.as_ref().unwrap().value, 5);
            assert_eq!(root.right.as_ref().unwrap().value, 20);
            assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().value, 15);
            assert_eq!(root.right.as_ref().unwrap().right.as_ref().unwrap().value, 25);
        }

        {
            let left_grand_child = Some(Box::new(Node::new(5, None, None)));
            let left_child = Some(Box::new(Node::new(10, left_grand_child, Some(Box::new(Node::new(15, None, None))))));
            let mut root = Node::new(20, left_child, Some(Box::new(Node::new(25, None, None))));

            root.rotate_ll();

            assert_eq!(root.height, 3);
            assert_eq!(root.left.as_ref().unwrap().height, 1);
            assert_eq!(root.right.as_ref().unwrap().height, 2);
        }
    }

    #[test]
    fn test_rotate_rr() {
        {
            let left_grand_child = Some(Box::new(Node::new(5, None, None)));
            let right_grand_child = Some(Box::new(Node::new(25, None, None)));
            let right_child = Some(Box::new(Node::new(20, Some(Box::new(Node::new(15, None, None))), right_grand_child)));
            let mut root = Node::new(10, left_grand_child, right_child);

            root.rotate_rr();

            assert_eq!(root.value, 20);
            assert_eq!(root.left.as_ref().unwrap().value, 10);
            assert_eq!(root.right.as_ref().unwrap().value, 25);
            assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().value, 5);
            assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap().value, 15);
        }

        {
            let left_grand_child = Some(Box::new(Node::new(5, None, None)));
            let right_child = Some(Box::new(Node::new(20, Some(Box::new(Node::new(15, None, None))), Some(Box::new(Node::new(25, None, None))))));
            let mut root = Node::new(10, left_grand_child, right_child);

            root.rotate_rr();

            assert_eq!(root.height, 3);
            assert_eq!(root.left.as_ref().unwrap().height, 2);
            assert_eq!(root.right.as_ref().unwrap().height, 1);
        }
    }

    #[test]
    fn test_balance() {
        {
            let mut root = Node::new(3, None, None);
            root.left = Some(Box::new(Node::new(2, None, None)));
            root.left.as_mut().unwrap().left = Some(Box::new(Node::new(1, None, None)));
            root.height = 3;
            root.left.as_mut().unwrap().height = 2;
            root.left.as_mut().unwrap().left.as_mut().unwrap().height = 1;

            root.balance();

            assert_eq!(root.value, 2);
            assert_eq!(root.height, 2);
            assert_eq!(root.left.as_ref().unwrap().value, 1);
            assert_eq!(root.left.as_ref().unwrap().height, 1);
            assert_eq!(root.right.as_ref().unwrap().value, 3);
            assert_eq!(root.right.as_ref().unwrap().height, 1);
        }

        {
            let mut root = Node::new(1, None, None);
            root.right = Some(Box::new(Node::new(2, None, None)));
            root.right.as_mut().unwrap().right = Some(Box::new(Node::new(3, None, None)));
            root.height = 3;
            root.right.as_mut().unwrap().height = 2;
            root.right.as_mut().unwrap().right.as_mut().unwrap().height = 1;

            root.balance();

            assert_eq!(root.value, 2);
            assert_eq!(root.height, 2);
            assert_eq!(root.left.as_ref().unwrap().value, 1);
            assert_eq!(root.left.as_ref().unwrap().height, 1);
            assert_eq!(root.right.as_ref().unwrap().value, 3);
            assert_eq!(root.right.as_ref().unwrap().height, 1);
        }

        {
            let mut root = Node::new(3, None, None);
            root.left = Some(Box::new(Node::new(1, None, None)));
            root.left.as_mut().unwrap().right = Some(Box::new(Node::new(2, None, None)));
            root.height = 3;
            root.left.as_mut().unwrap().height = 2;
            root.left.as_mut().unwrap().right.as_mut().unwrap().height = 1;

            root.balance();

            assert_eq!(root.value, 2);
            assert_eq!(root.height, 2);
            assert_eq!(root.left.as_ref().unwrap().value, 1);
            assert_eq!(root.left.as_ref().unwrap().height, 1);
            assert_eq!(root.right.as_ref().unwrap().value, 3);
            assert_eq!(root.right.as_ref().unwrap().height, 1);
        }

        {
            let mut root = Node::new(1, None, None);
            root.right = Some(Box::new(Node::new(3, None, None)));
            root.right.as_mut().unwrap().left = Some(Box::new(Node::new(2, None, None)));
            root.height = 3;
            root.right.as_mut().unwrap().height = 2;
            root.right.as_mut().unwrap().left.as_mut().unwrap().height = 1;

            root.balance();

            assert_eq!(root.value, 2);
            assert_eq!(root.height, 2);
            assert_eq!(root.left.as_ref().unwrap().value, 1);
            assert_eq!(root.left.as_ref().unwrap().height, 1);
            assert_eq!(root.right.as_ref().unwrap().value, 3);
            assert_eq!(root.right.as_ref().unwrap().height, 1);
        }
    }

    #[test]
    fn test_avl_tree() {
        let avl_tree = AVLTree::new(Vec::from([3, 7, 4, 6, 5, 1, 10, 2, 9, 8]));

        assert_eq!(avl_tree.root.as_ref().unwrap().value, 4);

        assert_eq!(
            avl_tree.root.as_ref().unwrap().left.as_ref().unwrap().value,
            2
        );

        assert_eq!(
            avl_tree.root.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().unwrap()
                .value,
            1
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().is_none(),
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().is_none(),
        );

        assert_eq!(
            avl_tree.root.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().unwrap()
                .value,
            3
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().is_none(),
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().is_none(),
        );

        assert_eq!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .value,
            7
        );

        assert_eq!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .value,
            6
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().is_none()
        );

        assert_eq!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().unwrap()
                .value,
            5
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().is_none()
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .right.as_ref().is_none()
        );

        assert_eq!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .value,
            9
        );

        assert_eq!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .value,
            8
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().unwrap()
                .left.as_ref().is_none()
        );

        assert_eq!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .value,
            10
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .left.as_ref().is_none()
        );
        assert!(
            avl_tree.root.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().unwrap()
                .right.as_ref().is_none()
        );
    }
}
