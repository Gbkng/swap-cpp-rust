use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use swap::naive_swap;

fn swap_vecs(c: &mut Criterion) {
    let mut group = c.benchmark_group("swap");

    for vec_size in [1, 1_000 + 1, 1_000_000 + 1] {
        let mut foo = vec![0; vec_size];
        let mut bar = vec![1; vec_size];
        group.bench_with_input(BenchmarkId::new("vec_size", vec_size), &vec_size, |b, _| {
            b.iter(|| {
                std::mem::swap(&mut foo, &mut bar);
            });
        });

        assert!(*foo.first().unwrap() == 1);
        assert!(*bar.first().unwrap() == 0);

        assert!(*foo.last().unwrap() == 1);
        assert!(*bar.last().unwrap() == 0);
    }
}
fn naive_swap_vecs(c: &mut Criterion) {
    let mut group = c.benchmark_group("naive_swap");

    for vec_size in [1, 1_000 + 1, 1_000_000 + 1] {
        let mut foo = vec![0; vec_size];
        let mut bar = vec![1; vec_size];
        group.bench_with_input(BenchmarkId::new("vec_size", vec_size), &vec_size, |b, _| {
            b.iter(|| {
                naive_swap(&mut foo, &mut bar);
            });

            assert!(*foo.first().unwrap() == 1);
            assert!(*bar.first().unwrap() == 0);

            assert!(*foo.last().unwrap() == 1);
            assert!(*bar.last().unwrap() == 0);
        });
    }
}
criterion_group!(benches, swap_vecs, naive_swap_vecs);
criterion_main!(benches);
