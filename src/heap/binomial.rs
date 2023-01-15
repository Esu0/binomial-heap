#[allow(dead_code)]
mod binomial_tree {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    struct Node<T> {
        val: T,
        degree: usize,
        bro: NodePtr<T>,
        child: NodePtr<T>,
    }

    type NodePtr<T> = Rc<RefCell<Option<Node<T>>>>;
    type NodePtrWeak<T> = Weak<RefCell<Option<Node<T>>>>;

    struct BinomialHeap<T> {
        head: NodePtr<T>,
        min: NodePtr<T>,
        tail: NodePtr<T>,
        size: usize,
    }

    impl<T> Node<T> {
        fn new(val: T, bro: NodePtr<T>) -> Self {
            Self {
                val,
                degree: 0,
                bro,
                child: Rc::new(RefCell::new(None)),
            }
        }

        fn ptr_null() -> NodePtr<T> {
            Rc::new(RefCell::new(None))
        }
    }

    impl<T> BinomialHeap<T> {
        pub fn new() -> Self {
            let p = Rc::new(RefCell::new(None));
            Self {
                min: Rc::clone(&p),
                tail: Rc::clone(&p),
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
            let new_node = Node::new(val, Rc::clone(&self.head));
            self.head = Rc::new(RefCell::new(Some(new_node)));
            if update_min {
                self.min = Rc::clone(&self.head);
            }
            self.size += 1;
        }

        fn consolidate(&mut self) {
            let mut nodelist = vec![
                Some(Node::<T>::ptr_null());
                self.size.next_power_of_two().trailing_zeros() as usize
            ];
            let ptr = Rc::clone(&self.head);
            // let ptr2 = Rc::clone(&ptr);
            loop {
                if ptr.borrow().is_none() {
                    break;
                }
                if let Some(p) = nodelist[ptr.borrow().as_ref().unwrap().degree].take() {
                    
                    self.meld(&ptr, &p);
                } else {
                    nodelist[ptr.borrow().as_ref().unwrap().degree] = Some(Rc::clone(&ptr));
                }
                
            }
        }

        fn meld(&mut self, p1: &NodePtr<T>, p2: &NodePtr<T>) {

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
}
