#![allow(dead_code)]

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;
use tokiobench::bench::tatlin;

use tokiobench::rt;

fn bench(name: &str, nspawn: &[usize], nspawner: &[usize], nworker: &[usize], c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
        let rt = rt::new(nworker, 1);

        group.throughput(Throughput::Elements((nspawn * nspawner) as u64));
        group.bench_function(
            format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
            |b| {
                let (root_handles, leaf_handles) = tatlin::mk_handles(nspawner, nspawn);

                b.iter_reuse(
                    (root_handles, leaf_handles),
                    |(root_handles, leaf_handles)| {
                        let tx = tx.clone();

                        let _gurad = rt.enter();
                        tatlin::run(nspawner, nspawn, tx, root_handles, leaf_handles);

                        rx.recv().unwrap()
                    },
                );
            },
        );
    }
    group.finish();
}

fn bench_origin(
    name: &str,
    nspawn: &[usize],
    nspawner: &[usize],
    nworker: &[usize],
    c: &mut Criterion,
) {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
        let rt = rt::new(nworker, 1);

        group.throughput(Throughput::Elements((nspawn * nspawner) as u64));
        group.bench_function(
            format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
            |b| {
                let (root_handles, leaf_handles) = tatlin::mk_handles(nspawner, nspawn);

                b.iter_reuse(
                    (root_handles, leaf_handles),
                    |(root_handles, leaf_handles)| {
                        let tx = tx.clone();

                        let _gurad = rt.enter();
                        tatlin::origin::run_origin(
                            nspawner,
                            nspawn,
                            tx,
                            root_handles,
                            leaf_handles,
                        );

                        rx.recv().unwrap()
                    },
                );
            },
        );
    }
    group.finish();
}

macro_rules! benches {
    ($expression:tt) => {
        pub fn local(c: &mut Criterion) {
            bench(
                concat!($expression, "/local"),
                &nspawn(),
                &nspawner(),
                &nworker(),
                c,
            )
        }

        pub fn local_origin(c: &mut Criterion) {
            bench_origin(
                concat!($expression, "/local"),
                &nspawn(),
                &nspawner(),
                &nworker(),
                c,
            )
        }
    };
}

fn nworker() -> Vec<usize> {
    vec![1, 2, 4, 8, 12, 16, 24]
}

fn nspawner() -> Vec<usize> {
    (1..=40).collect()
}

pub mod scatter {
    use super::*;

    fn nspawn() -> Vec<usize> {
        (1..=50).map(|i| i * 1000).collect()
    }

    benches! {"scatter"}
}

pub mod line {
    use super::*;

    fn nspawn() -> Vec<usize> {
        vec![5000]
    }

    benches! {"line"}
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(500)
        .measurement_time(Duration::from_secs(100))
        .warm_up_time(Duration::from_secs(3));

    targets = line::local_origin
);

criterion_main!(benches);
