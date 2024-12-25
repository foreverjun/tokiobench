use itertools::iproduct;

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tokiobench::rt;

use tokiobench::bench::remote;

fn bench(name: &str, nspawn: &[usize], nworker: &[usize], c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut group = c.benchmark_group(format!("remote/{name}"));

    for (&nspawn, &nworker) in iproduct!(nspawn, nworker) {
        let rt = rt::new(nworker);

        group.throughput(Throughput::Elements(nspawn as u64));
        group.bench_function(format!("nspawn({nspawn})/nworker({nworker})"), |b| {
            let handles = Vec::with_capacity(nspawn);
            b.iter_reuse(handles, |handles| {
                let tx = tx.clone();
                let _guard = rt.enter();

                remote::for_ch(nspawn, handles, tx);

                rx.recv().unwrap()
            });
        });
    }
    group.finish();
}

fn bench_const(c: &mut Criterion) {
    const NSPAWN: usize = 1000_000;

    let nworker: Vec<usize> = (2..=24).collect();

    bench("million", &[NSPAWN], nworker.as_ref(), c)
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(200)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(10));

    targets = bench_const
);

criterion_main!(benches);
