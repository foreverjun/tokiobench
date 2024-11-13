use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;
use std::sync::{mpsc, Arc};

use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use tokiobench::params;
use tokiobench::rt;

type BenchFn = fn(usize, SyncSender<()>, Arc<AtomicUsize>) -> ();

// from tokio
// spawn from current thread to inject queue
#[inline]
fn spawn_many_from_current(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    for _ in 0..nspawn {
        let tx = tx.clone();
        let rem = rem.clone();

        tokio::spawn(async move {
            if 1 == rem.fetch_sub(1, Relaxed) {
                tx.send(()).unwrap();
            }
        });
    }
}

// spawn from worker (proofe needed) to his local queue
// tasks must oveflow at some numbers
// and we should see this in graphs
#[inline]
fn spawn_many_from_local(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    tokio::spawn(async move {
        for _ in 0..nspawn {
            let rem = rem.clone();
            let tx = tx.clone();

            tokio::spawn(async move {
                if 1 == rem.fetch_sub(1, Relaxed) {
                    tx.send(()).unwrap();
                }
            });
        }
    });
}

fn bench_count_down(bench_fn: BenchFn, name: &str, c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1000);
    let rem: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    let mut group = c.benchmark_group(name);

    for (nspawn, nworkers) in iproduct!(params::NS_SPAWN, params::NS_WORKERS) {
        group.throughput(Throughput::Elements(nspawn as u64));
        group.bench_with_input(
            format!("nspawn({nspawn})/nwork({nworkers})"),
            &(nspawn, nworkers),
            |b, &(nspawn, nworkers)| {
                let rt = rt::new(nworkers);

                b.iter(|| {
                    let tx = tx.clone();
                    let rem = rem.clone();

                    rem.store(nspawn, Relaxed);
                    rt.block_on(async {
                        bench_fn(nspawn, tx, rem);

                        rx.recv().unwrap();
                    });
                });
            },
        );
    }
}

fn spawn_many_from_current_bench(c: &mut Criterion) {
    bench_count_down(spawn_many_from_current, "spawn_current", c)
}

fn spawn_many_from_local_bench(c: &mut Criterion) {
    bench_count_down(spawn_many_from_local, "spawn_local", c);
}

criterion_group!(
    spawn_benches,
    spawn_many_from_current_bench,
    spawn_many_from_local_bench
);

criterion_main!(spawn_benches);
