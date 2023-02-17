#![allow(dead_code)]

use std::alloc::Layout;
use std::marker::PhantomData;
// use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr::NonNull;
// use std::rc::Rc;

struct Node<K> {
    next: Option<NonNull<Node<K>>>,
    prev: Option<NonNull<Node<K>>>,
    child: Option<NonNull<Node<K>>>,
    degree: usize,
    key: K,
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
            degree: 0,
        }
    }

    fn new_ptr(key: K) -> NonNull<Self> {
        let layout = Layout::new::<Self>();
        unsafe {
            let raw = std::alloc::alloc(layout) as *mut Self;
            raw.write(Self::new(key));
            NonNull::new_unchecked(raw)
        }
    }

    fn new_cyclic(key: K) -> NonNull<Self> {
        let layout = Layout::new::<Self>();
        unsafe {
            let raw = std::alloc::alloc(layout) as *mut Self;
            let nn = NonNull::new(raw);
            raw.write(Self {
                key,
                next: nn,
                prev: nn,
                child: None,
                degree: 0,
            });
            // (*p.as_ptr()).next = Some(p);
            // (*p.as_ptr()).prev = Some(p);
            nn.unwrap()
        }
    }

    fn unlink(mut node: NonNull<Self>) -> NonNull<Self> {
        unsafe {
            if let Some(mut ne) = node.as_ref().next {
                node.as_mut().next = None;
                ne.as_mut().prev = None;
            }
            node
        }
    }

    /// nodeを指すポインタが残っていてはならない
    fn into_key(node: NonNull<Self>) -> K {
        unsafe {
            let n = std::ptr::read(node.as_ptr() as *const Self);
            std::alloc::dealloc(node.as_ptr() as *mut _, Layout::new::<Self>());
            n.key
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
    fn merge(mut root: NonNull<Node<K>>, mut other: NonNull<Node<K>>) -> NonNull<Node<K>> {
        unsafe {
            if root.as_ref().key > other.as_ref().key {
                if let Some(och) = other.as_ref().child {
                    (*other.as_ptr()).child = Some(Self::insert_node(och, root));
                } else {
                    root.as_mut().prev = Some(root);
                    root.as_mut().next = Some(root);
                    other.as_mut().child = Some(root);
                }
                other.as_mut().degree += 1;
                other
            } else {
                if let Some(rch) = root.as_ref().child {
                    (*root.as_ptr()).child = Some(Self::insert_node(rch, other));
                } else {
                    other.as_mut().prev = Some(other);
                    other.as_mut().next = Some(other);
                    root.as_mut().child = Some(other);
                }
                root.as_mut().degree += 1;
                root
            }
        }
    }

    fn insert_node(min: NonNull<Node<K>>, node: NonNull<Node<K>>) -> NonNull<Node<K>> {
        unsafe {
            Self::insert_node_next(min, node);
            if (*min.as_ptr()).key > (*node.as_ptr()).key {
                node
            } else {
                min
            }
        }
    }

    fn insert_node_prev(ptr: NonNull<Node<K>>, node: NonNull<Node<K>>) {
        unsafe {
            let prev = ptr.as_ref().prev.unwrap();
            (*prev.as_ptr()).next = Some(node);
            (*ptr.as_ptr()).prev = Some(node);
            (*node.as_ptr()).next = Some(ptr);
            (*node.as_ptr()).prev = Some(prev);
        }
    }

    fn insert_node_next(ptr: NonNull<Node<K>>, node: NonNull<Node<K>>) {
        unsafe {
            let next = ptr.as_ref().next.unwrap();
            (*next.as_ptr()).prev = Some(node);
            (*ptr.as_ptr()).next = Some(node);
            (*node.as_ptr()).prev = Some(ptr);
            (*node.as_ptr()).next = Some(next);
        }
    }

    pub fn insert(&mut self, key: K) {
        if let Some(nn) = self.min {
            unsafe {
                if nn.as_ref().key > key {
                    let new_node = Node::new_ptr(key);
                    Self::insert_node_prev(nn, new_node);
                    self.min = Some(new_node);
                } else {
                    let new_node = Node::new_ptr(key);
                    Self::insert_node_prev(nn, new_node);
                }
            }
        } else {
            self.min = Some(Node::new_cyclic(key));
        }
        self.size += 1;
    }

    /// Returns node whose key is minimum
    fn merge_list(
        head: NonNull<Node<K>>,
        head2: Option<NonNull<Node<K>>>,
        size: usize,
    ) -> NonNull<Node<K>> {
        let mut v = vec![None; size.ilog2() as usize + 1];
        let mut p = Some(head);
        unsafe {
            while let Some(mut ne) = p {
                p = (*ne.as_ptr()).next;
                while let Some(other) = v[(*ne.as_ptr()).degree].take() {
                    ne = Self::merge(ne, other);
                }
                v[(*ne.as_ptr()).degree] = Some(ne);
                if p == Some(head) {
                    break;
                }
            }
            p = head2;
            while let Some(mut ne) = p {
                p = (*ne.as_ptr()).next;
                while let Some(other) = v[(*ne.as_ptr()).degree].take() {
                    ne = Self::merge(ne, other);
                }
                v[(*ne.as_ptr()).degree] = Some(ne);
                if p == head2 {
                    break;
                }
            }
            let mut itr = v.into_iter().filter_map(|e| e);
            let head = itr.next().unwrap();
            let mut min = head;
            let mut p = head;
            for item in itr {
                (*p.as_ptr()).next = Some(item);
                (*item.as_ptr()).prev = Some(p);
                p = item;
                if (*min.as_ptr()).key > (*item.as_ptr()).key {
                    min = item;
                }
            }
            (*p.as_ptr()).next = Some(head);
            (*head.as_ptr()).prev = Some(p);
            min
        }
    }

    pub fn delete_min(&mut self) -> Option<K> {
        if let Some(min) = self.min {
            unsafe {
                let prev = (*min.as_ptr()).prev.unwrap();
                if prev == min {
                    self.min = (*min.as_ptr()).child;
                } else {
                    (*prev.as_ptr()).next = None;
                    self.min = Some(Self::merge_list(
                        (*min.as_ptr()).next.unwrap(),
                        (*min.as_ptr()).child,
                        self.size,
                    ));
                }
                self.size -= 1;
                Some(Node::into_key(min))
            }
        } else {
            None
        }
    }
}

use std::fmt::{self, Debug, Display};

impl<K: Display> Node<K> {
    fn fmt_tree(
        node: Option<NonNull<Node<K>>>,
        f: &mut fmt::Formatter<'_>,
        spaces: usize,
    ) -> fmt::Result {
        unsafe {
            let mut n = node;
            while let Some(p) = n {
                writeln!(f, "{}{}", "\t".repeat(spaces), (*p.as_ptr()).key)?;
                Self::fmt_tree((*p.as_ptr()).child, f, spaces + 1)?;
                n = (*p.as_ptr()).next;
                if n == node { break; }
            }
            Ok(())
        }
    }
}

impl<K: Display> Debug for BinomialHeap<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Node::fmt_tree(self.min, f, 0)
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use std::cell::Cell;
    #[test]
    #[allow(unused_assignments, unused_mut, unused_variables)]
    fn variant_test() {
        {
            let r_static: &'static i32 = &1;
            let num = 1;
            let r_local = &num;
            let vec_s: Vec<&'static i32> = vec![r_static];
            let mut vec_l = vec![r_local];
            vec_l = vec_s;
        }
        {
            let r_static: &'static i32 = &1;
            let num = 1;
            let r_local = &num;
            let cell_s: Cell<&'static i32> = Cell::new(r_static);
            let mut cell_l = Cell::new(r_local);
            // `num` does not live long enough
            // cell_l = cell_s;
        }
        {
            fn func_s(r_s: &'static i32) {
                println!("{}", r_s);
            }
            fn func_l(r_l: &i32) {
                println!("{}", r_l);
            }
            let r_static: &'static i32 = &1;
            let num = 1;
            let mut fn_s: fn(&'static i32) = func_s;
            let mut fn_l: fn(&i32) = func_l;
            fn_s = fn_l;
            fn_s(&100);
        }
    }

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct IntDebug(i32);

    impl Display for IntDebug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Drop for IntDebug {
        fn drop(&mut self) {
            println!("drop {}", self.0);
        }
    }

    use rand::Rng;

    #[test]
    fn heaptest() {
        let mut heap = BinomialHeap::new();
        let mut rng = rand::thread_rng();
        for _ in 0..30 {
            heap.insert(rng.gen_range(0..100));
        }
        for _ in 0..10 {
            print!("{} ", heap.delete_min().unwrap());
        }

        for _ in 0..10 {
            heap.insert(rng.gen_range(-50..50));
        }

        for i in 0..heap.len() {
            if i % 20 == 0 {
                println!();
            }
            print!("{} ", heap.delete_min().unwrap());
        }
    }
}
