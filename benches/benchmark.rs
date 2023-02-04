use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use heaps::heap::pairing::PairingHeap;
fn heaptest(c: &mut Criterion) {
    c.bench_function("test", |b| {
        b.iter(|| {
            let _h = criterion::black_box(PairingHeap::<i32>::new());
        })
    });
}

fn heapsort_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("heapsort");
    for n in (1..20).map(|n| (1 << n) as usize) {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| heaps::bench_fn::heapsort(n))
        });
    }
    group.finish();
}
criterion_group!(benches, heaptest, heapsort_test);
criterion_main!(benches);
