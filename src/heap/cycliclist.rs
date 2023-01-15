use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[allow(dead_code)]
struct Node<T> {
    key: T,
    prev: RefCell<Option<Rc<Node<T>>>>,
    next: RefCell<Weak<Node<T>>>,
}

#[allow(dead_code)]
pub struct CyclicList<T> {
    head: Rc<Node<T>>,
    tail: Rc<Node<T>>,
    size: usize,
}

#[allow(dead_code)]
pub struct Indexer<T>(Rc<Node<T>>);

impl<T> Clone for Indexer<T> {
    fn clone(&self) -> Self {
        Indexer(Rc::clone(&self.0))
    }
}

#[allow(dead_code)]
impl<T> Node<T> {
    fn new(key: T, prev: Option<Rc<Node<T>>>, next: Weak<Node<T>>) -> Self {
        Self {
            key,
            prev: RefCell::new(prev),
            next: RefCell::new(next),
        }
    }

    fn link_next(&self, node: Weak<Node<T>>) {
        *self.next.borrow_mut() = node;
    }

    fn unlink_next(&self) {
        *self.next.borrow_mut() = Weak::new();
    }

    fn link_prev(&self, node: Rc<Node<T>>) {
        *self.prev.borrow_mut() = Some(node);
    }

    fn unlink_prev(&self) {
        *self.prev.borrow_mut() = None;
    }
}

impl<T> Indexer<T> {
    pub fn key(&self) -> &T {
        &self.0.key
    }
}

impl<T> Indexer<T> {
    pub fn into_key(this: Self) -> Result<T, Self> {
        Rc::try_unwrap(this.0)
            .map(|n| n.key)
            .map_err(|p| Indexer(p))
    }
}

#[allow(dead_code)]
impl<T> CyclicList<T> {
    pub fn new(key: T) -> Self {
        let new_node = Rc::new(Node::new(key, None, Weak::new()));
        Self {
            head: Rc::clone(&new_node),
            tail: new_node,
            size: 1,
        }
    }

    pub fn take_one(&self) -> Indexer<T> {
        Indexer(Rc::clone(&self.head))
    }

    pub fn next(&self, node: &Indexer<T>) -> Indexer<T> {
        if let Some(n) = node.0.next.borrow().upgrade() {
            Indexer(n)
        } else {
            Indexer(Rc::clone(&self.head))
        }
    }

    pub fn prev(&self, node: &Indexer<T>) -> Indexer<T> {
        if let Some(n) = node.0.prev.borrow().as_ref() {
            Indexer(Rc::clone(n))
        } else {
            Indexer(Rc::clone(&self.tail))
        }
    }

    // Returns inserted node
    pub fn insert_next(&mut self, pos: &Indexer<T>, key: T) -> Indexer<T> {
        if Rc::ptr_eq(&pos.0, &self.tail) {
            let new_tail = Rc::new(Node::new(key, Some(Rc::clone(&self.tail)), Weak::new()));
            self.tail.link_next(Rc::downgrade(&new_tail));
            self.tail = Rc::clone(&new_tail);
            self.size += 1;
            Indexer(new_tail)
        } else {
            let old_next = pos.0.next.borrow().upgrade().unwrap();
            let new_node = Rc::new(Node::new(
                key,
                Some(Rc::clone(&pos.0)),
                Rc::downgrade(&old_next),
            ));
            pos.0.link_next(Rc::downgrade(&new_node));
            old_next.link_prev(Rc::clone(&new_node));
            self.size += 1;
            Indexer(new_node)
        }
    }

    pub fn insert_prev(&mut self, pos: &Indexer<T>, key: T) -> Indexer<T> {
        if Rc::ptr_eq(&pos.0, &self.head) {
            let new_head = Rc::new(Node::new(key, None, Rc::downgrade(&self.head)));
            self.head.link_prev(Rc::clone(&new_head));
            self.head = Rc::clone(&new_head);
            self.size += 1;
            Indexer(new_head)
        } else {
            let old_prev = Rc::clone(pos.0.prev.borrow().as_ref().unwrap());
            let new_node = Rc::new(Node::new(
                key,
                Some(Rc::clone(&old_prev)),
                Rc::downgrade(&pos.0),
            ));
            old_prev.link_next(Rc::downgrade(&new_node));
            pos.0.link_prev(Rc::clone(&new_node));
            self.size += 1;
            Indexer(new_node)
        }
    }

    // returns next node
    pub fn delete(&mut self, pos: &Indexer<T>) -> Indexer<T> {
        if self.size == 1 {
            pos.clone()
        } else if Rc::ptr_eq(&self.head, &pos.0) {
            let new_head = self.head.next.replace(Weak::new()).upgrade().unwrap();
            *new_head.prev.borrow_mut() = None;
            self.head = Rc::clone(&new_head);
            self.size -= 1;
            Indexer(new_head)
        } else if Rc::ptr_eq(&self.tail, &pos.0) {
            let new_tail = self.tail.prev.replace(None).unwrap();
            *new_tail.next.borrow_mut() = Weak::new();
            self.tail = new_tail;
            self.size -= 1;
            Indexer(Rc::clone(&self.head))
        } else {
            let next = pos.0.next.replace(Weak::new()).upgrade().unwrap();
            let prev = pos.0.prev.replace(None).unwrap();
            *prev.next.borrow_mut() = Rc::downgrade(&next);
            *next.prev.borrow_mut() = Some(prev);
            self.size -= 1;
            Indexer(next)
        }
    }

    // pub fn combine(&mut self, other: CyclicList<T>) {
    //     let CyclicList{ head, tail , size: _} = other;
    //     *self.tail.next.borrow_mut() = Rc::downgrade(&head);
    //     *head.prev.borrow_mut() = Some(Rc::clone(&self.tail));
    //     self.tail = tail;
    // }

    pub const fn len(&self) -> usize {
        self.size
    }
}

use std::fmt;

impl<T: fmt::Display> fmt::Debug for CyclicList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut node = Indexer(Rc::clone(&self.head));
        for _ in 0..self.size {
            write!(f, "{}, ", node.0.key)?;
            node = self.next(&node);
        }
        write!(f, "]")
    }
}
#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn cycliclist() {
        let mut list = CyclicList::new(1);
        let mut iter = list.take_one();
        println!("{:?}", &list);
        list.insert_prev(&iter, 3);
        println!("{:?}", &list);
        list.insert_next(&iter, 10);
        println!("{:?}", &list);
        iter = list.next(&iter);
        list.insert_next(&iter, 11);
        println!("{:?}", &list);
        list.insert_prev(&iter, 15);
        println!("{:?}", &list);
        let iter2 = list.delete(&iter);
        println!("{:?}", &list);
        println!("{}", Indexer::into_key(iter).unwrap_or_default());
    }
}
