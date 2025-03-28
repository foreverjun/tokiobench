#![allow(dead_code)]

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;

use tokiobench::bench::tatlin;
use tokiobench::rt;
use tokiobench::split::EqSplit;

fn bench(
    name: &str,
    nspawn: &[usize],
    nspawner: &[usize],
    nworker: &[usize],
    ngroup: &[usize],
    c: &mut Criterion,
) {
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker, &ngroup) in iproduct!(nspawn, nspawner, nworker, ngroup) {
        group.throughput(Throughput::Elements((nspawn * nspawner) as u64));
        group.sampling_mode(criterion::SamplingMode::Linear);

        let nspawner_p_group = EqSplit::new(nspawner, ngroup).item();
        let nworker_p_group = EqSplit::new(nworker, ngroup).item();
        let rt = rt::new_fixed(nworker_p_group, ngroup, 1);

        let rx_tx: Vec<_> = (0..ngroup).map(|_| mpsc::sync_channel(1)).collect();
        let _guard = rt.enter();

        group.bench_function(
            format!("ngroup({ngroup})/nworker({nworker})/nspawner({nspawner})/nspawn({nspawn})",),
            |b| {
                b.iter(|| {
                    for (group, (tx, _)) in rx_tx.iter().enumerate() {
                        let tx = tx.clone();
                        tatlin::fixed::run(nspawner_p_group, nspawn, tx, group);
                    }

                    for (_, rx) in rx_tx.iter() {
                        rx.recv().unwrap()
                    }
                });
            },
        );
    }
    group.finish();
}

fn nworker() -> Vec<usize> {
    vec![24]
}

fn nspawner() -> Vec<usize> {
    (1..=10).map(|i| i * 16).collect()
}

fn nruntime() -> Vec<usize> {
    vec![1, 2, 4, 8]
}

macro_rules! benches {
    ($expression:tt) => {
        pub fn sharded(c: &mut Criterion) {
            bench(
                concat!($expression, "/origin"),
                &nspawn(),
                &nspawner(),
                &nworker(),
                &nruntime(),
                c,
            )
        }
    };
}

pub mod line {
    use super::*;

    fn nspawn() -> Vec<usize> {
        vec![1000]
    }

    benches! {"line"}
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(100))
        .warm_up_time(Duration::from_secs(5));

    targets = line::sharded
);

criterion_main!(benches);
