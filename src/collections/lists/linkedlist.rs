use crate::node::{Node, NodePtr, NodePtrOpt};
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct List<T: std::marker::Copy + std::cmp::PartialEq> {
    head: NodePtrOpt<T>, // head node, the node at the begging of the list
    tail: NodePtrOpt<T>, // tail node, the node at the end of the list
    count: usize // the amount of elements in the list
}

impl<T: std::marker::Copy + std::cmp::PartialEq> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            count: 0
        }
    }

    // adds element to begging of list
    pub fn prepend(&mut self, element: T) -> &mut Self {
        let new_node = Some(Node::new(element));

        match self.head.take() {
            Some(old_head) => {
                new_node.as_ref().unwrap().borrow_mut().set_next_node(Some(old_head));
                self.head = new_node;
            },

            None => {
                self.head = new_node;
                if self.tail.is_none() {
                    self.tail = Some(Rc::clone(self.head.as_ref().unwrap()));
                }
            },
        }

        self.count += 1;
        self
    }

    // add element to the end of the list
    pub fn append(&mut self, element: T) -> &mut Self{
        let new_node = Node::new(element);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().set_next_node(Some(Rc::clone(&new_node)));
                self.tail = Some(new_node);
            },

            None => {
                self.tail = Some(new_node);
                if self.head.is_none() {
                    self.head = Some(Rc::clone(self.tail.as_ref().unwrap()));
                }
            },
        }

        self.count += 1;
        self
    }

    pub fn size(&self) -> usize {
        self.count
    }

    fn iter_node(&self) -> ListNodeIterator<T> {
        match &self.head {
            Some(head) => {
                ListNodeIterator::new(Some(Rc::clone(head)))
            },
            None => ListNodeIterator::new(None)
        }
    }

    pub fn iter(&self) -> ListIterator<T> {
        match &self.head {
            Some(head) => {
                ListIterator::new(Some(Rc::clone(head)))
            },
            None => ListIterator::new(None)
        }
    }

    pub fn count(&self, element: T) -> usize {
        let mut count = 0;
        for x in self.iter() {
            if x == element {
                count += 1;
            }
        }
        count
    }

    pub fn index(&self, element: T) -> Option<usize> {
        let mut i = 0;
        for x in self.iter() {
            if x == element {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn extend(&mut self, list: &List<T>) -> &mut Self {
        for element in list.iter() {
            self.append(element);
        }

        self
    }

    pub fn insert(&mut self, index: usize, element: T) -> &mut Self {
        if index > self.count {
            return self;
        }

        // special case insert end of list
        if index == self.count {
            self.count += 1;
            match self.tail.as_ref() {
                Some(mut tail) => {
                    let new = Node::new(element);
                    tail.borrow_mut().set_next_node(Some(Rc::clone(&new)));
                    self.tail = Some(Rc::clone(&new));
                },
                None => {
                    self.append(element);
                }
            }
            return self;
        }

        // special case insert at begging of list
        if index == 0 {
            self.count += 1;
            match self.head.as_ref() {
                Some(head) => {
                    let new = Node::new(element);
                    new.borrow_mut().set_next_node(Some(Rc::clone(head)));
                    self.head = Some(Rc::clone(&new));
                },
                None => {
                    self.append(element);
                }
            }
            return self;
        }

        // normal case
        self.count += 1;

        let mut i = 0;
        for mut node in self.iter_node() {
            if i == index-1 {
                let next = node.borrow_mut().get_next();
                let new = Node::new(element);
                new.borrow_mut().set_next_node(next);
                node.borrow_mut().set_next_node(Some(Rc::clone(&new)));

                return self;
            }
            i += 1;
        }

        // for what ever reason if this function fails, do nothing and return self to enable chaining
        self
    }

    pub fn remove(&mut self, element: T) {unimplemented!();}

    pub fn remove_at(&mut self, index: usize) {unimplemented!();}

    pub fn reverse(&mut self) {unimplemented!();}

    pub fn has(&self, element: T) -> Option<usize> {unimplemented!();}

    pub fn sort(&mut self) {unimplemented!();}

    pub fn clear(&mut self) {unimplemented!();}

    pub fn is_empty(&self) -> bool {unimplemented!();}
}

impl<T: std::marker::Copy + std::cmp::PartialEq> From<Vec<T>> for List<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut list = List::new() as List<T>;
        for x in vec {
            list.append(x);
        }
        list
    }
}

struct ListNodeIterator<T> {
    current: NodePtrOpt<T>
}

pub struct ListIterator<T> {
    node_iter: ListNodeIterator<T>
}

impl<T> ListNodeIterator<T> {
    pub fn new(start_at: NodePtrOpt<T>) -> Self {
        ListNodeIterator {
            current: start_at
        }
    }
}

impl<T> ListIterator<T> {
    pub fn new(start_at: NodePtrOpt<T>) -> Self {
        ListIterator {
            node_iter: ListNodeIterator::new(start_at)
        }
    }
}

impl<T> Iterator for ListNodeIterator<T> {
    type Item = NodePtr<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                result = Some(Rc::clone(current));
                match &current.borrow().get_next() {
                    Some(next_node) => {
                        Some(Rc::clone(next_node))
                    },
                    None => None,
                }
            },
            None => None,
        };

        result
    }
}

impl<T: Copy> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.node_iter.next() {
            Some(n) => {
                Some(n.borrow_mut().element)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn list_new() {
        let list = List::new() as List<i32>;
        assert_eq!(list, List {
            head: None,
            tail: None,
            count: 0,
        } as List<i32>);
    }

    #[test]
    fn list_prepend() {
        let mut list = List::new() as List<i32>;
        list.prepend(3);
        list.prepend(2);
        list.prepend(1);
        assert_eq!(list.count, 3);
        assert_ne!(list.head.as_ref().unwrap(), list.tail.as_ref().unwrap());
        assert_eq!(list.head.unwrap().borrow_mut().element, 1);
        assert_eq!(list.tail.unwrap().borrow_mut().element, 3);
    }

    #[test]
    fn list_append() {
        let mut list = List::new() as List<i32>;
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.count, 3);
        assert_ne!(list.head.as_ref().unwrap(), list.tail.as_ref().unwrap());
        assert_eq!(list.head.unwrap().borrow_mut().element, 1);
        assert_eq!(list.tail.unwrap().borrow_mut().element, 3);
    }

    #[test]
    fn list_size() {
        let mut list = List::new() as List<i32>;
        list.append(1);
        list.append(2);
        list.append(4);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn list_chain() {
        let mut list = List::new() as List<i32>;
        list.append(1).prepend(0).append(2);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn list_iter_node() {
        let mut list = List::new() as List<i32>;
        list.append(1).append(2).append(3);
        let mut count = 1;
        for x in list.iter_node() {
            assert_eq!(x.borrow_mut().element, count);
            count += 1;
        }
    }

    #[test]
    fn list_iter() {
        let mut list = List::new() as List<i32>;
        list.append(1).append(2).append(3);
        let mut count = 1;
        for x in list.iter() {
            assert_eq!(x, count);
            count += 1;
        }
    }

    #[test]
    fn list_count() {
        let mut list = List::new() as List<i32>;
        list.append(1).append(2).append(3).append(2);
        assert_eq!(list.count(0), 0);
        assert_eq!(list.count(2), 2);
        assert_eq!(list.count(3), 1);
    }

    #[test]
    fn list_index() {
        let mut list = List::new() as List<i32>;
        list.append(1).append(2).append(3);
        assert_eq!(list.index(1).unwrap(), 0);
        assert_eq!(list.index(3).unwrap(), 2);
        assert_eq!(list.index(10).is_none(), true);
    }

    #[test]
    fn list_from_vec() {
        let mut vec = vec![0, 1, 2, 3];
        let mut list = List::from(vec.clone());
        assert_eq!(list.size(), 4);
        let mut i =0;
        for x in list.iter() {
            assert_eq!(x, vec[i]);
            i += 1;
        }
    }

    #[test]
    fn list_extend() {
        let mut list1 = List::from(vec![0, 1, 2]);
        let mut list2 = List::from(vec![3, 4, 5]);
        list1.extend(&list2);
        assert_eq!(list1, List::from(vec![0, 1, 2, 3, 4, 5]));
        list2.extend(&List::from(vec![0, 1, 2]));
        assert_eq!(list2, List::from(vec![3, 4, 5, 0, 1, 2]));
    }

    #[test]
    fn list_insert() {
        let mut list = List::from(vec![0, 1, 3, 4, 5]);
        list.insert(2, 2).insert(6, 6);
        assert_eq!(list, List::from(vec![0, 1, 2, 3, 4, 5, 6]));
        list.insert(0, 10).insert(8, 7);
        assert_eq!(list, List::from(vec![10, 0, 1, 2, 3, 4, 5, 6, 7]));
        list.insert(3, 100);
        assert_eq!(list, List::from(vec![10, 0, 1, 100, 2, 3, 4, 5, 6, 7]));
    }
}