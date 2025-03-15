#![allow(dead_code)]

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use tokiobench::bench::tatlin;
use tokiobench::rt;
use tokiobench::split::EqSplit;

const NGROUP: usize = 4;
const NWORKER: usize = 32;
const NSPAWN: usize = 10000;

fn bench(name: &str, nspawner: &[usize], c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for &nspawner in nspawner {
        group.sampling_mode(criterion::SamplingMode::Linear);
        group.throughput(Throughput::Elements((NSPAWN * nspawner) as u64));

        {
            let nspawner_p_group = EqSplit::new(nspawner, NGROUP).item();
            let nworker_p_group = EqSplit::new(NWORKER, NGROUP).item();
            let rt = rt::new_shard(nworker_p_group, NGROUP, 1);

            let rx_tx: Vec<_> = (0..NGROUP).map(|_| mpsc::sync_channel(1)).collect();

            group.bench_with_input(BenchmarkId::new("grouped", nspawner), &(), |b, _| {
                b.iter(|| {
                    for (group, (tx, _)) in rx_tx.iter().enumerate() {
                        let tx = tx.clone();
                        rt.spawn_into(
                            async move { tatlin::sharded::run(nspawner_p_group, NSPAWN, tx) },
                            group,
                        );
                    }

                    for (_, rx) in rx_tx.iter() {
                        rx.recv().unwrap()
                    }
                })
            });
        }
        {
            let (tx, rx) = mpsc::sync_channel(1);
            let rt = rt::new_ref(NWORKER, 1);

            group.bench_with_input(BenchmarkId::new("reference", nspawner), &(), |b, _| {
                b.iter(|| {
                    let _guard = rt.enter();
                    tatlin::reference::run(nspawner, NSPAWN, tx.clone());
                    rx.recv().unwrap()
                })
            });
        }
    }
    group.finish();
}

fn nspawner() -> Vec<usize> {
    (1..=10).map(|i| i * 16).collect()
}

macro_rules! benches {
    ($expression:tt) => {
        pub fn sharded(c: &mut Criterion) {
            bench(concat!($expression, "/origin"), &nspawner(), c)
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
