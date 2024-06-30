use std::fmt::Display;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T> {
   fn new(v: T) -> Self {
    Node {
        value: v,
        left: None,
        right: None
    }
   } 
}

#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<Node<T>>>
}

impl<T: Ord + Display> BinaryTree<T> {

    fn new() -> Self {
        BinaryTree {
            root: None
        }
    }

    fn insert(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        match self.root {
            None => self.root = Some(new_node),
            Some(ref mut node) => Self::insert_node( node, new_node),
        }
    }

    fn insert_node(current: &mut Box<Node<T>>, new_node: Box<Node<T>>) {
        if new_node.value < current.value {
            match current.left {
                None => current.left = Some(new_node),
                Some(ref mut left_child) => Self::insert_node(left_child, new_node)
            }
        } else {
            match current.right {
                None => current.right = Some(new_node),
                Some(ref mut right_child) => Self::insert_node(right_child, new_node)
            }
        }
    }

    fn in_order_traversal(&self) {
        Self::in_order(&self.root);
        println!()
    }

    fn in_order(node: &Option<Box<Node<T>>>) {
        if let Some(ref n) = node {
            Self::in_order(&n.left);
            print!("{} ", n.value);
            Self::in_order(&n.right);
        }
    }
}


pub fn run() {
    let mut tree = BinaryTree::new();

    tree.insert(70);
    tree.insert(10);
    tree.insert(5);
    tree.insert(50);
    
    println!("In-order traversal");
    tree.in_order_traversal()
}