use itertools::iproduct;

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tokiobench::rt;

use tokiobench::bench::remote;

fn join_all(name: &str, nspawn: &[usize], nworker: &[usize], c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut group = c.benchmark_group(format!("remote/{name}"));

    for (&nspawn, &nworker) in iproduct!(nspawn, nworker) {
        let rt: tokio::runtime::Runtime = rt::new(nworker);

        group.throughput(Throughput::Elements(nspawn as u64));
        group.bench_function(format!("nspawn({nspawn})/nworker({nworker})"), |b| {
            b.iter(|| {
                let tx = tx.clone();
                let _guard = rt.enter();

                remote::join_all(nspawn, tx);

                rx.recv().unwrap();
            });
        });
    }
    group.finish();
}

fn bench_join_all(c: &mut Criterion) {
    const NSPAWN: usize = 1000_000;

    let nworker: Vec<usize> = (2..=24).collect();

    join_all("million", &[NSPAWN], nworker.as_ref(), c)
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(200)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(10));

    targets = bench_join_all
);

criterion_main!(benches);
