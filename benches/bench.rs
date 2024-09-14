use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use memset_pattern::memset_pattern;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill");

    for src_len in 1..=64 {
        group.bench_with_input(
            BenchmarkId::from_parameter(src_len),
            &src_len,
            |b, &src_len| {
                let src = vec![0; src_len];
                let mut dst = vec![0; 1000000];

                b.iter(|| {
                    memset_pattern(black_box(&mut dst), black_box(&src));
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
