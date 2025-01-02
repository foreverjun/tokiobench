#![allow(dead_code)]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;
use tokiobench::bench::tatlin;

use tokiobench::rt;

pub mod builder {
    use super::*;

    pub mod join_all {
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
                let rt = rt::new(nworker, 0);

                group.throughput(Throughput::Elements(nspawn as u64));
                group.bench_function(
                    format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
                    |b| {
                        b.iter(|| {
                            let tx = tx.clone();

                            let _gurad = rt.enter();
                            tatlin::join_all::tx(nspawner, nspawn, tx);

                            rx.recv().unwrap();
                        });
                    },
                );
            }
            group.finish();
        }

        pub fn spin(
            name: &str,
            nspawn: &[usize],
            nspawner: &[usize],
            nworker: &[usize],
            c: &mut Criterion,
        ) {
            let end = Arc::new(AtomicBool::new(false));
            let mut group = c.benchmark_group(format!("tatlin/{name}"));

            for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
                let rt = rt::new(nworker, 0);

                group.throughput(Throughput::Elements(nspawn as u64));
                group.bench_function(
                    format!("nspawn({nspawn})/nspawner({nspawner})/nworker({nworker})"),
                    |b| {
                        b.iter(|| {
                            end.store(false, Relaxed);

                            let _guard = rt.enter();
                            tatlin::join_all::spin(nspawner, nspawn, Arc::clone(&end));

                            while !end.load(Acquire) {
                                std::hint::spin_loop();
                            }
                        });
                    },
                );
            }
            group.finish();
        }
    }

    pub mod vec {
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
                let rt = rt::new(nworker, 0);

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
                                tatlin::vec::ch(nspawner, nspawn, tx, root_handles, leaf_handles);

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
                            let leaf_handles = (0..nspawner)
                                .map(|_| Vec::with_capacity(nspawn))
                                .collect::<Vec<_>>();
                            let root_handles = Vec::with_capacity(nspawner);

                            b.iter_reuse(
                                (root_handles, leaf_handles),
                                |(root_handles, leaf_handles)| {
                                    let tx = tx.clone();

                                    let _gurad = rt.enter();
                                    tatlin::vec::blocking::ch(
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

        // TODO (duplication (but no curring!!!))
        pub mod no_lifo {
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
                    let rt = rt::new_no_lifo(nworker, 0);

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
                                    tatlin::vec::blocking::ch(
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

        pub mod join_all {
            use super::*;

            pub fn ch(c: &mut Criterion) {
                builder::join_all::ch("scatter/join_all/ch", &nspawn(), &nspawner(), &nworker(), c)
            }

            pub fn spin(c: &mut Criterion) {
                builder::join_all::spin(
                    "scatter/join_all/spin",
                    &nspawn(),
                    &nspawner(),
                    &nworker(),
                    c,
                )
            }
        }
        pub mod vec {
            use super::*;

            pub fn ch(c: &mut Criterion) {
                builder::vec::ch("scatter/vec/ch", &nspawn(), &nspawner(), &nworker(), c)
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
        pub mod vec {
            use super::*;

            pub fn ch(c: &mut Criterion) {
                builder::vec::ch("line/vec/ch", &nspawn(), &nspawner(), &nworker(), c)
            }

            pub mod no_lifo {
                use super::*;

                pub fn ch(c: &mut Criterion) {
                    builder::vec::no_lifo::ch(
                        "line/vec/no_lifo/ch",
                        &nspawn(),
                        &nspawner(),
                        &nworker(),
                        c,
                    )
                }
            }

            pub mod blocking {
                use super::*;

                pub fn ch(c: &mut Criterion) {
                    builder::vec::blocking::ch(
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
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(200)
        .measurement_time(Duration::from_secs(40))
        .warm_up_time(Duration::from_secs(3));

    targets = bench::line::vec::blocking::ch
);

criterion_main!(benches);
