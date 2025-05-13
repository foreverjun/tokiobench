#![allow(dead_code)]

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;
use tokiobench::bench::tatlin;

use tokiobench::rt;

fn bench(
    name: &str,
    fun: tatlin::Bench,
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
        group.sampling_mode(criterion::SamplingMode::Linear);

        group.bench_function(
            format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
            |b| {
                b.iter(|| {
                    let _guard = rt.enter();

                    fun(nspawner, nspawn, tx.clone());
                    rx.recv().unwrap()
                });
            },
        );
    }
    group.finish();
}

fn nworker() -> Vec<usize> {
    vec![ 4, 8, 16, 32, 48]
}

fn nspawner() -> Vec<usize> {
    (2..=32).step_by(2).collect()
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
        vec![3000]
    }

    benches! {"line"}
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(40)
        .measurement_time(Duration::from_secs(80))
        .warm_up_time(Duration::from_secs(5));

    targets = line::cleaned
);

criterion_main!(benches);
