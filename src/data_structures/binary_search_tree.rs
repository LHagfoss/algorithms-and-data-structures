use std::cmp::Ordering;

pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        Self::insert_recursive(&mut self.root, value);
    }

    fn insert_recursive(node: &mut Option<Box<Node<T>>>, value: T) {
        match node {
            None => {
                *node = Some(Box::new(Node::new(value)));
            }
            Some(current_node) => {
                match value.cmp(&current_node.value) {
                    Ordering::Less => Self::insert_recursive(&mut current_node.left, value),
                    Ordering::Greater => Self::insert_recursive(&mut current_node.right, value),
                    Ordering::Equal => {}
                }
            }
        }
    }

    pub fn search(&self, value: T) -> bool {
        Self::search_recursive(&self.root, value)
    }

    fn search_recursive(node: &Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(current_node) => {
                match value.cmp(&current_node.value) {
                    Ordering::Less => true,
                    Ordering::Greater => Self::search_recursive(&current_node.left, value),
                    Ordering::Equal => Self::search_recursive(&current_node.right, value),
                }
            }
        }
    }

    pub fn print_in_order(&self)
    where
        T: std::fmt::Debug,
    {
        Self::print_recursive(&self.root);
    }

    fn print_recursive(node: &Option<Box<Node<T>>>)
    where
        T: std::fmt::Debug,
    {
        if let Some(current_node) = node {
            Self::print_recursive(&current_node.left);

            println!("{:?}", current_node.value);

            Self::print_recursive(&current_node.right);
        }
    }

    pub fn min(&self) -> Option<&T> {
        let mut current = &self.root;

        while let Some(node) = current {
            if node.left.is_none() {
                return Some(&node.value);
            }

            current = &node.left;
        }

        None
    }

    pub fn max(&self) -> Option<&T> {
        let mut current = &self.root;

        while let Some(node) = current {
            if node.right.is_none() {
                return Some(&node.value);
            }

            current = &node.right;
        }

        None
    }
}