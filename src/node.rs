use std::cell::{RefCell};
use std::rc::Rc;

// RC allows multiple ownership, RefCell allows interior mutability.
pub type NodePtr<T> = Rc<RefCell<Node<T>>>;
pub type NodePtrOpt<T> = Option<NodePtr<T>>;


#[derive(PartialEq, Debug)]
pub struct Node<T> {
    pub element: T,
    next: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    // creates a node and returns it as an rc pointer
    pub fn new(element: T) -> NodePtr<T> {
        Rc::new(RefCell::new(Self {
            element,
            next: None
        }))
    }

    pub fn set_next(&mut self, element: T) {
        self.next = Some(Node::new(element));
    }

    pub fn get_next(&self) -> NodePtrOpt<T> {
        match &self.next {
            Some(next) => Some(next.clone()),
            None => None
        }
    }

    pub fn set_next_node(&mut self, node: NodePtrOpt<T>) {
        self.next = node;
    }
}


#[cfg(test)]
mod tests {
    use super::Node;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn node_new() {
        let n = Node::new(10);
        assert_eq!(n, Rc::new(RefCell::new(Node{
            element: 10,
            next: None,
        })));
    }

    #[test]
    fn node_clone() {
        // multiple references WOOT! :D
        let n = Node::new(10);
        let n1 = n.clone();
        assert_eq!(n, n1);
    }

    #[test]
    fn node_set_and_get_next() {
        let n = Node::new(1);
        n.borrow_mut().set_next(2);
        assert_eq!(n.borrow_mut().get_next().unwrap().borrow_mut().element, 2);
    }

    #[test]
    fn node_set_next_node() {
        let n = Node::new(1);
        let n1 = Node::new(2);
        n.borrow_mut().set_next_node(Some(Rc::clone(&n1)));
        assert_eq!(n.borrow_mut().get_next().unwrap().borrow_mut().element,
                   2);
        n1.borrow_mut().set_next_node(Some(Node::new(3)));
        assert_eq!(n1.borrow_mut().get_next().unwrap().borrow_mut().element, 3);
    }

    #[test]
    fn node_into_inner() {
        let n = Node::new(1);
        let t = n.as_ref().borrow_mut().element;
        assert_eq!(t, 1);
    }
}