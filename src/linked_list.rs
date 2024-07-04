#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        return Self {
            value,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        return Self {
            head: None,
            length: 0,
        };
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        let mut current = &self.head;

        for _ in 0..index {
            if let Some(node) = current {
                current = &node.next;
            }
        }

        if let Some(node) = current.as_deref() {
            return Some(&node.value);
        }

        return None;
    }

    pub fn push(&mut self, value: T) {
        let node = Some(Box::new(Node::new(value)));

        match &mut self.head {
            None => self.head = node,
            Some(_) => {
                let last_node = self.get_mut(self.length - 1);

                last_node.unwrap().next = node;
            },
        }

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        } else if self.length == 1 {
            if let Some(node) = self.head.take() {
                self.length -= 1;
                return Some(node.value);
            }
        }

        let mut return_value = None;

        if let Some(node) = self.get_mut(self.length - 2) {
            if let Some(next_node) = node.next.take() {
                return_value = Some(next_node.value);
                node.next = None;
            }
        }

        self.length -= 1;

        return return_value;

    }

    pub fn delete(&mut self, index: usize) {
        if self.length == 0 {
            return;
        }

        if index == 0 {
            if let Some(mut node) = self.head.take() {
                self.head = node.next.take();
            }
        } else {
            let prev_node = self.get_mut(index - 1);

            if let Some(prev) = prev_node {
                let node_to_remove = prev.next.take();

                if let Some(mut node) = node_to_remove {
                    prev.next = node.next.take();
                }
            }
        }

        self.length -= 1;
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone
    {
        let mut current = &self.head;
        let mut result = Vec::with_capacity(self.length);

        while let Some(node) = current {
            result.push(node.value.clone());
            current = &node.next;
        }

        return result;
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        if index >= self.length {
            return None;
        }

        let mut current = &mut self.head;

        for _ in 0..index {
            if let Some(ref mut node) = current {
                current = &mut node.next;
            }
        }

        return current.as_deref_mut();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut linked_list = LinkedList::new();

        linked_list.push(1);
        linked_list.push(2);
        linked_list.push(3);
        assert_eq!(linked_list.to_vec(), [1, 2, 3]);

        linked_list.pop();
        linked_list.pop();
        assert_eq!(linked_list.to_vec(), [1]);

        linked_list.push(2);
        linked_list.push(3);
        linked_list.delete(1);
        assert_eq!(linked_list.to_vec(), [1, 3]);
        assert_eq!(linked_list.get(1), Some(&3));
    }
}
