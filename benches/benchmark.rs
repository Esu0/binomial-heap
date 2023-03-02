use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use heaps::heap::pairing::PairingHeap;
fn heaptest(c: &mut Criterion) {
    c.bench_function("test", |b| {
        b.iter(|| {
            let _h = criterion::black_box(PairingHeap::<i32>::new());
        })
    });
}

fn pairing_heapsort(c: &mut Criterion) {
    let mut group = c.benchmark_group("heapsort");
    for n in (1..20).map(|n| (1 << n) as usize) {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| heaps::bench_fn::pairing::heapsort(n))
        });
    }
    group.finish();
}

fn binomial_heapsort(c: &mut Criterion) {
    let mut group = c.benchmark_group("heapsort - binomial heap");
    for n in (1..20).map(|n| (1 << n) as usize) {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| heaps::bench_fn::binomial::heapsort(n))
        });
    }
    group.finish();
}

criterion_group!(benches, heaptest, pairing_heapsort, binomial_heapsort);
criterion_main!(benches);
