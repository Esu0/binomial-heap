use std::cell::RefCell;

#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Option<RefCell<T>>,
}

#[allow(dead_code)]
struct List<T> {
    head: Option<RefCell<T>>,
    tail: Option<RefCell<T>>,
    size: usize,
}

struct Indexer<T> {
    pos: RefCell<T>,
}
#[allow(dead_code)]
impl<T> Node<T> {
    const fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

#[allow(dead_code)]
impl<T> List<T> {
    const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push_back(&mut self, value: T) {}
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use std::ptr::NonNull;
    #[test]
    fn ptr_test() {
        let ptr;
        {
            let mut x = [100, 200, 300];
            ptr = &mut x as *mut i32;
        }
        let np = NonNull::new(ptr).unwrap();
        let a = [20, 30, 40, 50, 60];
        unsafe {
            println!("{}", *(np.as_ptr().wrapping_add(1)));
        }
    }

    use std::collections::LinkedList;
    #[test]
    fn linkedlist_test() {
        let mut list = LinkedList::new();
        list.push_front(20);
        list.push_back(30);
        list.push_back(35);
        let r1 = list.front_mut().unwrap();
        //let r2 = list.front_mut().unwrap();
        *r1 = 3;
    }

    // #[test]
    // fn lifetime_test() {
    //     let l
    // }
}
