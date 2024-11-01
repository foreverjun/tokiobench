use tokio::runtime::{self, Runtime};

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{mpsc, Arc};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// const STALL_DUR: Duration = Duration::from_micros(10); TODO(add stall)

const NWORKERS: [usize; 7] = [1, 2, 4, 6, 8, 10, 12];

// TODO(we must sheck spawning in bound of local queue and above to ensure contantion overhead)
// const NTASKS: [usize; 6] = [10, 100, 1000, 10000, 10000, 100000]; TODO(change number of tasks)
// const NITER: [usize; 6] = [1, 5, 10, 15, 20, 25]; TODO(change number of iter)

// fix for now
const NITER: usize = 10;
const NSPAWN: usize = 100000;
const NALL: usize = NITER * NSPAWN;

fn rt(workers: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(workers)
        .enable_all()
        .build()
        .unwrap()
}

// from tokio
fn rt_multi_spawn_many_from_current(c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1000);
    let rem = Arc::new(AtomicUsize::new(0));

    let mut group = c.benchmark_group("spawn_many_from_current");

    for workers in NWORKERS.iter() {
        group.throughput(Throughput::Elements(*workers as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(workers),
            workers,
            |b, &workers| {
                let rt = rt(workers);

                // TODO(decouple function to dump metrics)
                b.iter(|| {
                    let tx = tx.clone();
                    let rem = rem.clone();

                    rem.store(NALL, Relaxed);

                    rt.block_on(async {
                        for _ in 0..NITER {
                            // TODO() cpu bound task here?
                            for _ in 0..NSPAWN {
                                let tx = tx.clone();
                                let rem = rem.clone();

                                tokio::spawn(async move {
                                    if 1 == rem.fetch_sub(1, Relaxed) {
                                        tx.send(()).unwrap();
                                    }
                                });
                            }
                        }

                        rx.recv().unwrap();
                    });
                });
            },
        );
    }
}

pub fn rt_multi_spawn_many_to_local(c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1000);
    let rem = Arc::new(AtomicUsize::new(0));

    let mut group = c.benchmark_group("spawn_many_to_local");

    for workers in NWORKERS.iter() {
        group.throughput(Throughput::Elements(*workers as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(workers),
            workers,
            |b, &workers| {
                let rt = rt(workers);

                // TODO(decouple function to dump metrics)
                b.iter(|| {
                    let tx = tx.clone();
                    let rem = rem.clone();

                    rem.store(NALL, Relaxed);

                    rt.block_on(async {
                        tokio::spawn(async move {
                            for _ in 0..NITER {
                                // TODO() cpu bound task here?
                                for _ in 0..NSPAWN {
                                    let rem = rem.clone();
                                    let tx = tx.clone();

                                    tokio::spawn(async move {
                                        if 1 == rem.fetch_sub(1, Relaxed) {
                                            tx.send(()).unwrap();
                                        }
                                    });
                                }
                            }
                        });

                        rx.recv().unwrap();
                    });
                });
            },
        );
    }
}

criterion_group!(
    spawn_benches,
    rt_multi_spawn_many_from_current,
    rt_multi_spawn_many_to_local
);

criterion_main!(spawn_benches);
