use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;
use tokiobench::bench::tatlin;

use tokiobench::rt;

fn _ch(name: &str, nspawn: &[usize], nspawner: &[usize], nworker: &[usize], c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
        let rt = rt::new(nworker);

        group.throughput(Throughput::Elements(nspawn as u64));
        group.bench_function(
            format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
            |b| {
                b.iter(|| {
                    let tx = tx.clone();

                    let _gurad = rt.enter();
                    tatlin::tx(nspawner, nspawn, tx);

                    rx.recv().unwrap();
                });
            },
        );
    }
    group.finish();
}

fn _spin(name: &str, nspawn: &[usize], nspawner: &[usize], nworker: &[usize], c: &mut Criterion) {
    let end = Arc::new(AtomicBool::new(false));
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
        let rt = rt::new(nworker);

        group.throughput(Throughput::Elements(nspawn as u64));
        group.bench_function(
            format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
            |b| {
                b.iter(|| {
                    end.store(false, Relaxed);

                    let _guard = rt.enter();
                    tatlin::spin(nspawner, nspawn, Arc::clone(&end));

                    while !end.load(Acquire) {
                        std::hint::spin_loop();
                    }
                });
            },
        );
    }
    group.finish();
}

fn for_ch(name: &str, nspawn: &[usize], nspawner: &[usize], nworker: &[usize], c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
        let rt = rt::new(nworker);

        group.throughput(Throughput::Elements(nspawn as u64));
        group.bench_function(
            format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
            |b| {
                let leaf_handles = (0..nspawner)
                    .map(|_| Vec::with_capacity(nspawn))
                    .collect::<Vec<_>>();
                let root_handles = Vec::with_capacity(nspawner);

                b.iter_reuse(
                    (root_handles, leaf_handles),
                    |(root_handles, leaf_handles)| {
                        let tx = tx.clone();

                        let _gurad = rt.enter();
                        tatlin::for_await_ch(nspawner, nspawn, tx, root_handles, leaf_handles);

                        rx.recv().unwrap()
                    },
                );
            },
        );
    }
    group.finish();
}

fn _bench_ch(c: &mut Criterion) {
    let nspawn: Vec<usize> = (1..=10).map(|i| i * 1000).collect();
    let nspawner: Vec<usize> = (1..=20).collect();
    let nworker: Vec<usize> = (1..=10).map(|i| i * 2).collect();

    _ch("ch", &nspawn, &nspawner, &nworker, c)
}

fn _bench_spin(c: &mut Criterion) {
    let nspawn: Vec<usize> = (1..=10).map(|i| i * 1000).collect();
    let nspawner: Vec<usize> = (1..=20).collect();
    let nworker: Vec<usize> = (1..=10).map(|i| i * 2).collect();

    _spin("spin", &nspawn, &nspawner, &nworker, c)
}

fn _bench_for_ch(c: &mut Criterion) {
    let nspawn: Vec<usize> = (1..=10).map(|i| i * 1000).collect();
    let nspawner: Vec<usize> = (1..=20).collect();
    let nworker: Vec<usize> = (1..=10).map(|i| i * 2).collect();

    for_ch("for_ch", &nspawn, &nspawner, &nworker, c)
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(300)
        .measurement_time(Duration::from_secs(30))
        .warm_up_time(Duration::from_secs(3));

    targets = _bench_for_ch, _bench_ch, _bench_spin
);

criterion_main!(benches);
