#![allow(dead_code)]

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;
use tokiobench::bench::tatlin;

use tokiobench::rt;
use tokiobench::split::Split;

fn bench(
    name: &str,
    fun: tatlin::Bench,
    nspawn: &[usize],
    nspawner_total: &[usize],
    nworker: &[usize],
    nruntime: &[usize],
    c: &mut Criterion,
) {
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner_total, &nworker, &nruntime) in
        iproduct!(nspawn, nspawner_total, nworker, nruntime)
    {
        let runtimes: Vec<_> = (0..nruntime).map(|_| rt::new(nworker, 1)).collect();

        let mut txs = Vec::with_capacity(nruntime);
        let mut rxs = Vec::with_capacity(nruntime);
        for _ in 0..nruntime {
            let (tx, rx) = mpsc::sync_channel(1);
            txs.push(tx);
            rxs.push(rx);
        }

        let nspawners: Vec<_> = Split::new(nspawner_total, nruntime).collect();

        group.throughput(Throughput::Elements((nspawn * nspawner_total) as u64));
        group.sampling_mode(criterion::SamplingMode::Linear);

        group.bench_function(
            format!("nruntime({nruntime})/nworker({nworker})/nspawner({nspawner_total})/nspawn({nspawn})"),
            |b| {
                b.iter(|| {
                    for i in 0..nruntime {
                        let tx = txs[i].clone();
                        let nspawner = nspawners[i];

                        runtimes[i].spawn(async move { fun(nspawner, nspawn, tx) });
                    }

                    for rx in rxs.iter() {
                        rx.recv().unwrap()
                    }
                });
            },
        );
    }
    group.finish();
}

fn nworker() -> Vec<usize> {
    vec![1, 2, 4, 8]
}

fn nspawner() -> Vec<usize> {
    (1..=10).map(|i| i * 16).collect()
}

fn runtimes() -> Vec<usize> {
    vec![1, 2, 4]
}

macro_rules! benches {
    ($expression:tt) => {
        pub fn origin(c: &mut Criterion) {
            bench(
                concat!($expression, "/origin"),
                tatlin::origin::run,
                &nspawn(),
                &nspawner(),
                &nworker(),
                &runtimes(),
                c,
            )
        }

        pub fn cleaned(c: &mut Criterion) {
            bench(
                concat!($expression, "/cleaned"),
                tatlin::cleaned::run,
                &nspawn(),
                &nspawner(),
                &nworker(),
                &runtimes(),
                c,
            )
        }
    };
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

    targets = line::origin
);

criterion_main!(benches);
