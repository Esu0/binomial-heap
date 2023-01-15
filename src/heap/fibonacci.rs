use super::cycliclist::{CyclicList, Indexer};

#[allow(dead_code)]
struct Node {
    children: Vec<Node>,
    key: i32,
}

#[allow(dead_code)]
struct FibonacciHeap {
    min: Option<(CyclicList<Node>, Indexer<Node>)>,
    size: usize,
}

#[allow(dead_code)]
impl Node {
    const fn new(key: i32) -> Self {
        Self {
            children: Vec::new(),
            key,
        }
    }

    fn key(&self) -> &i32 {
        &self.key
    }
}

#[allow(dead_code)]
impl FibonacciHeap {
    pub const fn new() -> Self {
        Self { min: None, size: 0 }
    }

    pub fn insert(&mut self, key: i32) {
        self.add_root(Node::new(key));
        self.size += 1;
    }

    fn add_root(&mut self, root: Node) {
        if let Some((l, i)) = &mut self.min {
            let i2 = l.insert_next(i, root);
            if i.key().key() > i2.key().key() {
                *i = i2;
            }
        } else {
            let list = CyclicList::new(root);
            let i = list.take_one();
            self.min = Some((list, i));
        }
    }
}
