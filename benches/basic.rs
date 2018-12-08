#[macro_use]
extern crate criterion;
extern crate assets_benchmark;
extern crate uuid;

use assets_benchmark::{transfer_tcp, transfer_uds};
use criterion::{Criterion, Fun};

fn criterion_benchmark(c: &mut Criterion) {
    let uds = Fun::new("Tokio UDS", |b, _i| {
        b.iter(|| {
            let path = format!("/tmp/assets-benchmark-{}", uuid::Uuid::new_v4().to_string());
            transfer_uds(path);
        })
    });
    let tcp = Fun::new("Tokio TCP", |b, _i| {
        b.iter(|| {
            let addr = "127.0.0.1:0".parse().unwrap();
            transfer_tcp(&addr)
        })
    });

    let functions = vec![uds, tcp];

    c.bench_functions("10mb", functions, 1);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
