#![allow(dead_code)]

use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::rc::Rc;
struct Node<K> {
    key: K,
    next: Option<NonNull<Node<K>>>,
    prev: Option<NonNull<Node<K>>>,
    child: Option<NonNull<Node<K>>>,
}

pub struct BinomialHeap<K> {
    min: Option<NonNull<Node<K>>>,
    size: usize,
    marker: PhantomData<Box<Node<K>>>,
}

impl<K> Node<K> {
    const fn new(key: K) -> Self {
        Self {
            key,
            next: None,
            prev: None,
            child: None,
        }
    }

    fn new_ptr(key: K) -> NonNull<Self> {
        Box::leak(Box::new(Self::new(key))).into()
    }

    fn new_cyclic(key: K) -> NonNull<Self> {
        let mut uninit_ptr = MaybeUninit::uninit();
        unsafe {
            let p = NonNull::new_unchecked(uninit_ptr.as_mut_ptr());
            uninit_ptr.write(Self {
                key,
                next: Some(p),
                prev: Some(p),
                child: None,
            });
            // (*p.as_ptr()).next = Some(p);
            // (*p.as_ptr()).prev = Some(p);
            p
        }
    }
}

impl<K> BinomialHeap<K> {
    pub const fn new() -> Self {
        Self {
            min: None,
            size: 0,
            marker: PhantomData,
        }
    }

    pub const fn len(&self) -> usize {
        self.size
    }
}

impl<K: Ord> BinomialHeap<K> {

    fn merge(root: NonNull<Node<K>>, other: NonNull<Node<K>>) {
        
    }
    pub fn insert(&mut self, key: K) {
        if let Some(nn) = self.min {
            
        } else {
            self.min = Some(Node::new_cyclic(key));
        }
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
}