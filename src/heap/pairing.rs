#![allow(dead_code)]

use std::marker::PhantomData;
use std::ptr::NonNull;
// use std::collections::LinkedList;
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    child: Option<NonNull<Node<T>>>,
}

pub struct PairingHeap<T> {
    head: Option<NonNull<Node<T>>>,
    size: usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Node<T> {
    const fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            child: None,
        }
    }

    fn new_ptr(val: T) -> *mut Node<T> {
        Box::leak(Box::new(Self {
            val,
            next: None,
            child: None,
        })) as *mut _
    }
}

impl<T: Ord> Node<T> {
    /// root.next and other.next must be None.
    #[must_use]
    fn merge(root: Option<NonNull<Self>>, other: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        match (root, other) {
            (Some(p1), Some(p2)) => unsafe {
                Some((&*Self::merge_unchecked(p1.as_ptr(), p2.as_ptr())).into())
            },
            (None, p) => p,
            (p, None) => p,
        }
    }

    /// root and other mustn't be null. \
    /// root.next and other.next must be None.
    #[must_use]
    fn merge_unchecked(root: *mut Self, other: *mut Self) -> *mut Self {
        unsafe {
            if (*root).val > (*other).val {
                (*root).next = (*other).child;
                (*other).child = Some((&*root).into());
                other
            } else {
                (*other).next = (*root).child;
                (*root).child = Some((&*other).into());
                root
            }
        }
    }

    #[must_use]
    fn merge_list(mut root: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        let mut newtree = std::ptr::null_mut::<Self>();
        unsafe {
            while let Some(mut ptr) = root.map(|wrapped| wrapped.as_ptr()) {
                root = (*ptr).next;
                (*ptr).next = None;
                if let Some(ptr2) = root.map(|wrapped| wrapped.as_ptr()) {
                    root = (*ptr2).next;
                    (*ptr2).next = None;
                    ptr = Node::merge_unchecked(ptr, ptr2);
                }
                (*ptr).next = Some((&*newtree).into());
                newtree = ptr;
            }

            if newtree.is_null() {
                None
            } else {
                let mut next = (*newtree).next;
                (*newtree).next = None;

                while let Some(ptr) = next.map(|wrapped| wrapped.as_ptr()) {
                    next = (*ptr).next;
                    (*ptr).next = None;
                    newtree = Node::merge_unchecked(newtree, ptr);
                }
                Some((&*newtree).into())
            }
        }
    }
}

impl<T: fmt::Display> Node<T> {
    fn fmt_rec(
        root: Option<NonNull<Self>>,
        spaces: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if let Some(root) = root.map(|wrapped| wrapped.as_ptr()) {
            unsafe {
                writeln!(f, "{}{}", "\t".repeat(spaces), (*root).val)?;
                Node::fmt_rec((*root).child, spaces + 1, f)?;
                Node::fmt_rec((*root).next, spaces, f)
            }
        } else {
            Ok(())
        }
    }
}

impl<T> PairingHeap<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            size: 0,
            marker: PhantomData,
        }
    }

    pub const fn len(&self) -> usize {
        self.size
    }
}

impl<T: Ord> PairingHeap<T> {
    pub fn insert(&mut self, val: T) {
        let node = Box::new(Node::new(val));
        let nodeptr = Box::leak(node).into();
        self.head = Node::merge(self.head, Some(nodeptr));
        self.size += 1;
    }

    pub fn delete_min(&mut self) -> Option<T> {
        if let Some(p) = self.head.map(|wrapped| wrapped.as_ptr()) {
            unsafe {
                let tmp = (*p).child;
                (*p).child = None;
                self.head = Node::merge_list(tmp);
                self.size -= 1;
                Some(Box::from_raw(p).val)
            }
        } else {
            None
        }
    }
}
use std::fmt;

impl<T: fmt::Display> fmt::Debug for PairingHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Node::fmt_rec(self.head, 0, f)
    }
}

impl<T: Clone> Clone for PairingHeap<T> {
    fn clone(&self) -> Self {
        let new_head = self.head.map(|nn| {
            let mut stack = Vec::with_capacity(self.size);
            unsafe {
                let new_head = Node::new_ptr((*nn.as_ptr()).val.clone());
                stack.push((nn.as_ptr(), new_head));
                while let Some((mut old, mut new)) = stack.pop() {
                    loop {
                        if let Some(ne) = (*old).next {
                            let new_node_ptr = Node::new_ptr((*ne.as_ptr()).val.clone());
                            (*new).next = Some(NonNull::new_unchecked(new_node_ptr));
                            stack.push((ne.as_ptr(), new_node_ptr));
                        }
                        if let Some(ch) = (*old).child {
                            let new_node_ptr = Node::new_ptr((*ch.as_ptr()).val.clone());
                            (*new).child = Some(NonNull::new_unchecked(new_node_ptr));
                            old = ch.as_ptr();
                            new = new_node_ptr;
                        } else {
                            break;
                        }
                    }
                }
                NonNull::new_unchecked(new_head)
            }
        });
        Self {
            head: new_head,
            size: self.size,
            marker: PhantomData,
        }
    }
}

impl<T> Drop for PairingHeap<T> {
    fn drop(&mut self) {
        if let Some(head) = self.head.map(|nn| nn.as_ptr()) {
            let mut stack = Vec::with_capacity(self.size);
            stack.push(head);
            while let Some(mut ptr) = stack.pop() {
                unsafe {
                    loop {
                        if let Some(ne) = (*ptr).next {
                            stack.push(ne.as_ptr());
                        }
                        if let Some(ch) = (*ptr).child {
                            std::ptr::drop_in_place(ptr);
                            ptr = ch.as_ptr();
                        } else {
                            std::ptr::drop_in_place(ptr);
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut heap = PairingHeap::new();
        heap.insert(100);
        heap.insert(200);
        heap.insert(250);
    }

    #[test]
    fn clone_test() {
        let mut heap = PairingHeap::new();
        for i in 0..50 {
            heap.insert(i);
        }

        heap.delete_min().unwrap();
        heap.delete_min().unwrap();
        heap.delete_min().unwrap();
        heap.delete_min().unwrap();

        println!("old: \n{:?}", &heap);
        println!("cloned: \n{:?}", heap.clone());
    }
}
