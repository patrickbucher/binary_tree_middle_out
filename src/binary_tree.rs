use crate::middle_out::middle_out;
use std::cmp::Ordering;

pub struct Node<T: Clone + Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

enum Traversal {
    InOrder,
    PreOrder,
}

impl<T> Node<T>
where
    T: Clone + Ord,
{
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => match &mut self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(Self::new(value))),
            },
            Ordering::Greater => match &mut self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(Self::new(value))),
            },
            Ordering::Equal => (),
        }
    }

    pub fn get_values(&self, order: &Traversal) -> Vec<T> {
        let left = match &self.left {
            Some(node) => node.get_values(order),
            None => Vec::new(),
        };
        let middle = vec![self.value.clone()];
        let right = match &self.right {
            Some(node) => node.get_values(order),
            None => Vec::new(),
        };
        match order {
            Traversal::InOrder => [left, middle, right].concat(),
            Traversal::PreOrder => [middle, left, right].concat(),
        }
    }

    pub fn delete(&self, value: &T) -> Option<Self> {
        let values = self.get_values(&Traversal::InOrder);
        let values: Vec<T> = values.into_iter().filter(|v| v != value).collect();
        let values = middle_out(&values);
        if values.is_empty() {
            None
        } else {
            let mut root = Self::new(values[0].clone());
            for v in &values[1..] {
                root.insert(v.clone());
            }
            Some(root)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_get_values() {
        let mut root: Node<usize> = Node::new(4);
        root.insert(2);
        root.insert(1);
        root.insert(3);
        root.insert(6);
        root.insert(5);
        root.insert(7);

        let expected: Vec<usize> = (1..=7).collect();
        let actual = root.get_values(&Traversal::InOrder);
        assert_eq!(actual, expected);

        let expected: Vec<usize> = vec![4, 2, 1, 3, 6, 5, 7];
        let actual = root.get_values(&Traversal::PreOrder);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_delete_leaf() {
        let mut root: Node<usize> = Node::new(4);
        root.insert(2);
        root.insert(1);
        root.insert(3);
        root.insert(6);
        root.insert(5);
        root.insert(7);

        let tree = root.delete(&3).unwrap();
        assert_eq!(tree.get_values(&Traversal::InOrder), vec![1, 2, 4, 5, 6, 7]);
    }

    #[test]
    fn test_delete_middle() {
        let mut root: Node<usize> = Node::new(4);
        root.insert(2);
        root.insert(1);
        root.insert(3);
        root.insert(6);
        root.insert(5);
        root.insert(7);

        let tree = root.delete(&6).unwrap();
        assert_eq!(tree.get_values(&Traversal::InOrder), vec![1, 2, 3, 4, 5, 7]);
    }

    #[test]
    fn test_delete_root() {
        let mut root: Node<usize> = Node::new(4);
        root.insert(2);
        root.insert(1);
        root.insert(3);
        root.insert(6);
        root.insert(5);
        root.insert(7);

        let tree = root.delete(&4).unwrap();
        assert_eq!(tree.get_values(&Traversal::InOrder), vec![1, 2, 3, 5, 6, 7]);
    }
}
