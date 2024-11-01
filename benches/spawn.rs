use tokio::runtime::{self, Runtime};

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;
use std::sync::{mpsc, Arc};

use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

// const STALL_DUR: Duration = Duration::from_micros(10); TODO(add stall)

const NWORKERS: [usize; 7] = [1, 2, 4, 6, 8, 10, 12];
const NSPAWN: [usize; 6] = [10, 100, 1000, 10000, 10000, 100000];

fn data() -> impl Iterator<Item = (usize, usize)> {
    iproduct!(NWORKERS, NSPAWN)
}

fn rt(workers: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(workers)
        .enable_all()
        .build()
        .unwrap()
}

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
fn spawn_many_to_local(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
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

#[inline]
fn bench_count_down(bench_fn: BenchFn, c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1000);
    let rem: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    let mut group = c.benchmark_group("spawn_many_from_current");

    for (nspawn, nworkers) in data() {
        group.throughput(Throughput::Elements(nspawn as u64));
        group.bench_with_input(
            format!("ns = {}/{} = nw", nworkers, nspawn),
            &(nspawn, nworkers),
            |b, &(nworkers, nspawn)| {
                let rt = rt(nworkers);

                b.iter(|| {
                    let tx = tx.clone();
                    let rem = rem.clone();

                    rem.store(nspawn, Relaxed);
                    // collect metrics here TODO()
                    rt.block_on(async {
                        bench_fn(nspawn, tx, rem);

                        rx.recv().unwrap();
                    });
                });
            },
        );
    }
}

fn rt_multi_spawn_many_from_current(c: &mut Criterion) {
    bench_count_down(spawn_many_from_current, c)
}

fn rt_multi_spawn_many_to_local(c: &mut Criterion) {
    bench_count_down(spawn_many_to_local, c);
}

criterion_group!(
    spawn_benches,
    rt_multi_spawn_many_from_current,
    rt_multi_spawn_many_to_local
);

criterion_main!(spawn_benches);
