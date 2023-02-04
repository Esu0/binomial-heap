#[allow(dead_code)]
mod binomial_tree {
    use std::cell::{Ref, RefCell, RefMut};
    use std::fmt;
    use std::rc::Rc;

    struct Node<T> {
        val: T,
        degree: usize,
        bro: NodePtr<T>,
        child: NodePtr<T>,
    }

    struct NodePtr<T>(Rc<RefCell<Option<Node<T>>>>);
    // type NodePtrWeak<T> = Weak<RefCell<Option<Node<T>>>>;

    struct BinomialHeap<T> {
        head: NodePtr<T>,
        min: NodePtr<T>,
        tail: NodePtr<T>,
        size: usize,
    }

    impl<T> NodePtr<T> {
        fn new(node: Node<T>) -> Self {
            Self(Rc::new(RefCell::new(Some(node))))
        }

        fn null() -> Self {
            Self(Rc::new(RefCell::new(None)))
        }

        fn borrow(&self) -> Ref<Option<Node<T>>> {
            self.0.borrow()
        }

        fn borrow_mut(&self) -> RefMut<Option<Node<T>>> {
            self.0.borrow_mut()
        }

        fn fmt_rec(&self, f: &mut fmt::Formatter<'_>, spaces: usize) -> fmt::Result
        where
            T: fmt::Display,
        {
            if let Some(n) = self.0.borrow().as_ref() {
                writeln!(f, "{}|{}\t\t", "\t\t".repeat(spaces), n.val)?;
                n.child.fmt_rec(f, spaces + 1)?;
                n.bro.fmt_rec(f, spaces)
            } else {
                Ok(())
            }
        }
    }

    impl<T> Clone for NodePtr<T> {
        fn clone(&self) -> Self {
            Self(Rc::clone(&self.0))
        }
    }

    impl<T> Node<T> {
        fn new(val: T, bro: NodePtr<T>) -> Self {
            Self {
                val,
                degree: 0,
                bro,
                child: NodePtr::null(),
            }
        }

        fn into_val(self) -> T {
            self.val
        }
    }

    impl<T> BinomialHeap<T> {
        pub fn new() -> Self {
            let p = NodePtr::null();
            Self {
                min: p.clone(),
                tail: p.clone(),
                head: p,
                size: 0,
            }
        }
    }

    impl<T: Ord> BinomialHeap<T> {
        fn insert(&mut self, val: T) {
            let update_min = self
                .min
                .borrow()
                .as_ref()
                .map(|n| n.val > val)
                .unwrap_or(true);
            let new_node = Node::new(val, self.head.clone());
            self.head = NodePtr::new(new_node);
            if update_min {
                self.min = self.head.clone();
            }
            self.size += 1;
        }

        fn delete_min(&mut self) -> T {
            let mut m = self.min.borrow_mut();
            let m_old = std::mem::replace(&mut *m, None);
            let m_old_ref = m_old.as_ref().unwrap();
            std::mem::swap(&mut *m, &mut *m_old_ref.bro.borrow_mut());
            std::mem::swap(
                &mut *self.tail.borrow_mut(),
                &mut *m_old_ref.child.borrow_mut(),
            );
            drop(m);
            self.consolidate();
            self.size -= 1;
            m_old.unwrap().into_val()
        }

        fn consolidate(&mut self) {
            let mut nodelist: Vec<Option<NodePtr<T>>> =
                vec![None; self.size.next_power_of_two().trailing_zeros() as usize];
            let mut ptr = self.head.clone();

            // let ptr2 = Rc::clone(&ptr);
            loop {
                self.tail = match ptr.borrow().as_ref() {
                    Some(n) => {
                        if self.min.borrow().as_ref().map(|m| m.val > n.val).unwrap_or(true) {
                            self.min = ptr.clone();
                        }
                        n.bro.clone()
                    }
                    None => {
                        break;
                    }
                };
                if {
                    let inner = ptr.borrow();
                    if let Some(p) = nodelist[inner.as_ref().unwrap().degree].take() {
                        drop(inner);
                        self.meld(ptr.clone(), p);
                        false
                    } else {
                        nodelist[inner.as_ref().unwrap().degree] = Some(ptr.clone());
                        true
                    }
                } {
                    ptr = self.tail.clone();
                }
                // https://github.com/rust-lang/rust/issues/70919
            }
        }

        fn meld(&mut self, p1: NodePtr<T>, p2: NodePtr<T>) -> NodePtr<T> {
            let mut wp1n = p1.borrow_mut();
            let mut wp2n = p2.borrow_mut();
            let p1n = wp1n.as_mut().unwrap();
            let p2n = wp2n.as_mut().unwrap();
            if p1n.val > p2n.val {
                // drop(p1n);
                p2n.degree += 1;
                let p2ncrc = p2n.child.clone();
                drop(p2n);
                drop(wp2n);
                let mut p2nc = p2ncrc.borrow_mut();
                std::mem::swap(&mut *wp1n, &mut *p2nc);
                std::mem::swap(&mut *p2nc.as_mut().unwrap().bro.borrow_mut(), &mut *wp1n);
                p2
            } else {
                // drop(p2n);
                p1n.degree += 1;
                let p1ncrc = p1n.child.clone();
                drop(p1n);
                drop(wp1n);
                let mut p1nc = p1ncrc.borrow_mut();
                std::mem::swap(&mut *wp2n, &mut *p1nc);
                let p1ncb = std::mem::replace(&mut p1nc.as_mut().unwrap().bro, p2.clone());
                std::mem::swap(&mut *p1nc.as_mut().unwrap().bro.borrow_mut(), &mut *wp2n);
                p1
            }
        }
    }

    impl<T: fmt::Display> fmt::Debug for BinomialHeap<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.head.fmt_rec(f, 0)?;
            if let Some(n) = self.min.borrow().as_ref() {
                writeln!(f, "minimum: {}", n.val)
            } else {
                writeln!(f, "minimum: None")
            }
        }
    }

    #[cfg(test)]
    mod test {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn heaptest() {
            let mut heap = BinomialHeap::new();
            heap.insert(10);
            heap.insert(20);
            heap.insert(30);
            println!("{:?}", &heap);
            println!("{}", heap.delete_min());
            println!("{:?}", &heap);
        }
    }
}
#[cfg(test)]
mod test {
    use std::cell::Cell;
    fn test<'a>(_arg1: &mut Vec<&'a i32>, _arg2: &mut Vec<&'a i32>) {}

    fn celltest<'a>(_a: Cell<&'a i32>, _b: Cell<&'a i32>) {}
    #[test]
    fn variant_test() {
        // let a: &'static i32 = &1;
        // let b = 2;
        // let c = &b;
        // let mut v1: Vec<&'static i32> = vec![a];
        // let mut v2 = vec![c];
        // test(&mut v1, &mut v2);
    }

    #[test]
    fn test2() {
        let a = 1;
        let cella = Cell::new(&a);
        {
            let b = 2;
            let cellb = Cell::new(&b);
            celltest(cella, cellb);
        }
        println!("{a}");
    }

    struct Test<const N: usize> {
        nums: [i32; N],
    }

    impl<const N: usize> Default for Test<N> {
        fn default() -> Self {
            Self { nums: [0; N] }
        }
    }

    fn ptr_to_address<T>(ptr: *const T) -> usize {
        unsafe { std::mem::transmute(ptr) }
    }

    fn ref_to_address<T>(reference: &T) -> usize {
        unsafe { std::mem::transmute(reference) }
    }

    #[test]
    fn ptr_test() {
        const N: usize = 1 << 15;
        let mut a = Test { nums: [0; N] };
        a.nums[0] = 10;
        let p = &a as *const Test<N>;
        let b = std::mem::take(&mut a);
        unsafe {
            println!("size of struct: {}", std::mem::size_of::<Test<N>>());
            println!("b.num: {}", b.nums[0]);
            println!("address of b: {}", ref_to_address(&b));
            println!("num in p: {}", (*p).nums[0]);
            println!("address of p: {}", ptr_to_address(p));
            println!("address of reference p: {}", ref_to_address(&(*p)));
        }
    }

    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn cell_test() {
        let cell = Rc::new(RefCell::new(30));
        let cell2 = Rc::clone(&cell);
        let mut ref1 = cell.borrow_mut();
        *ref1 = 4;
        drop(ref1);
        let mut ref2 = cell2.borrow_mut();
        *ref2 = 3;
        println!("{}", ref2);
    }

    #[test]
    fn rc_test() {
        let rc1 = Rc::new(100);
        let mut rc2 = Rc::new(15);
        println!("{}", rc2);
        rc2 = Rc::clone(&rc1);
        println!("{}", rc2);
    }
}
