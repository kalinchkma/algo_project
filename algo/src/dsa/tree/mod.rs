use std::rc::Rc;

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