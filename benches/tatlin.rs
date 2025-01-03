#![allow(dead_code)]

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;
use tokiobench::bench::tatlin;

use tokiobench::rt;

pub mod builder {
    use super::*;

    pub fn ch(
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

            group.throughput(Throughput::Elements(nspawn as u64));
            group.bench_function(
                format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
                |b| {
                    let (root_handles, leaf_handles) = tatlin::mk_handles(nspawner, nspawn);

                    b.iter_reuse(
                        (root_handles, leaf_handles),
                        |(root_handles, leaf_handles)| {
                            let tx = tx.clone();

                            let _gurad = rt.enter();
                            tatlin::ch(nspawner, nspawn, tx, root_handles, leaf_handles);

                            rx.recv().unwrap()
                        },
                    );
                },
            );
        }
        group.finish();
    }

    pub mod blocking {
        use super::*;

        pub fn ch(
            name: &str,
            nspawn: &[usize],
            nspawner: &[usize],
            nworker: &[usize],
            c: &mut Criterion,
        ) {
            let (tx, rx) = mpsc::sync_channel(1);
            let mut group = c.benchmark_group(format!("tatlin/{name}"));

            for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
                let rt = rt::new(nworker, nspawner);

                group.throughput(Throughput::Elements(nspawn as u64));
                group.bench_function(
                    format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
                    |b| {
                        let (root_handles, leaf_handles) = tatlin::mk_handles(nspawner, nspawn);

                        b.iter_reuse(
                            (root_handles, leaf_handles),
                            |(root_handles, leaf_handles)| {
                                let tx = tx.clone();

                                let _gurad = rt.enter();
                                tatlin::blocking::ch(
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
    }
}

fn nworker() -> Vec<usize> {
    vec![1, 2, 4, 8, 12, 16, 24]
}

mod bench {
    use super::*;

    pub mod scatter {
        use super::*;

        fn nspawn() -> Vec<usize> {
            (1..=10).map(|i| i * 1000).collect()
        }

        fn nspawner() -> Vec<usize> {
            (1..=20).collect()
        }

        pub fn ch(c: &mut Criterion) {
            builder::ch("scatter/vec/ch", &nspawn(), &nspawner(), &nworker(), c)
        }

        pub mod blocking {
            use super::*;

            pub fn ch(c: &mut Criterion) {
                builder::blocking::ch(
                    "line/vec/blocking/ch",
                    &nspawn(),
                    &nspawner(),
                    &nworker(),
                    c,
                )
            }
        }
    }

    pub mod line {
        use super::*;

        fn nspawn() -> Vec<usize> {
            vec![1000, 5000, 10000]
        }

        fn nspawner() -> Vec<usize> {
            (1..=20).collect()
        }

        pub fn ch(c: &mut Criterion) {
            builder::ch("line/vec/ch", &nspawn(), &nspawner(), &nworker(), c)
        }

        pub mod blocking {
            use super::*;

            pub fn ch(c: &mut Criterion) {
                builder::blocking::ch(
                    "line/vec/blocking/ch",
                    &nspawn(),
                    &nspawner(),
                    &nworker(),
                    c,
                )
            }
        }
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(200)
        .measurement_time(Duration::from_secs(40))
        .warm_up_time(Duration::from_secs(3));

    targets = bench::line::blocking::ch, bench::line::ch
);

criterion_main!(benches);
