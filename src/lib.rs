pub mod heap;

pub mod bench_fn {
    use super::heap::pairing::PairingHeap;
    use rand::Rng;
    pub fn heapsort(len: usize) {
        let mut heap = PairingHeap::new();
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            heap.insert(rng.gen_range(-100000000..100000000));
        }
        let _v: Vec<i32> = (0..len).map(|_| heap.delete_min().unwrap()).collect();
    }
}
