mod heap;
use heap::pairing::PairingHeap;
fn main() {
    let mut heap = PairingHeap::new();
    heap.insert(100);
    println!("Hello, world!");
}
