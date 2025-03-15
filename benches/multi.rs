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
    nruntime: &[usize],
    c: &mut Criterion,
) {
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker, &nruntime) in iproduct!(nspawn, nspawner, nworker, nruntime)
    {
        group.throughput(Throughput::Elements((nspawn * nspawner) as u64));
        group.sampling_mode(criterion::SamplingMode::Linear);

        let nspawner_p_rt = EqSplit::new(nspawner, nruntime).item();
        let nworker_p_rt = EqSplit::new(nworker, nruntime).item();

        let rt_rx_tx: Vec<_> = (0..nruntime)
            .map(|_| {
                let rt = rt::new_ref(nworker_p_rt, 1);
                let (tx, rx) = mpsc::sync_channel(1);

                (rt, tx, rx)
            })
            .collect();

        group.bench_function(
            format!(
                "nruntime({nruntime})/nworker({nworker})/nspawner({nspawner})/nspawn({nspawn})",
            ),
            |b| {
                b.iter(|| {
                    for (rt, tx, _) in rt_rx_tx.iter() {
                        let tx = tx.clone();
                        rt.spawn(async move { tatlin::reference::run(nspawner_p_rt, nspawn, tx) });
                    }

                    for (_, _, rx) in rt_rx_tx.iter() {
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
        pub fn origin(c: &mut Criterion) {
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

    targets = line::origin
);

criterion_main!(benches);
