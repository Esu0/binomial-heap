use criterion::{criterion_group, criterion_main, Criterion};

use heaps::heap::pairing::PairingHeap;
fn heaptest(c: &mut Criterion) {
    c.bench_function("test", |b| {
        b.iter(|| {
            let _h = criterion::black_box(PairingHeap::<i32>::new());
        })
    });
}

criterion_group!(benches, heaptest);
criterion_main!(benches);
