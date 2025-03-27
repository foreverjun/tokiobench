#![allow(dead_code)]

use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;

use tokiobench::bench::spawner;
use tokiobench::rt;

fn bench(name: &str, nspawn: &[usize], nspawner: &[usize], nworker: &[usize], c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
        group.throughput(Throughput::Elements((nspawn * nspawner) as u64));
        group.sampling_mode(criterion::SamplingMode::Linear);

        let rt = rt::new_id(nworker, 1);
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        let _guard = rt.enter();

        group.bench_function(
            format!("nworker({nworker})/nspawner({nspawner})/nspawn({nspawn})",),
            |b| {
                b.iter(|| {
                    let tx = tx.clone();
                    spawner::id::run(nspawner, nspawn, tx);

                    rx.recv().unwrap()
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

macro_rules! benches {
    ($expression:tt) => {
        pub fn id(c: &mut Criterion) {
            bench(
                concat!($expression, "/origin"),
                &nspawn(),
                &nspawner(),
                &nworker(),
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

    targets = line::id
);

criterion_main!(benches);
