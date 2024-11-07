use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;
use std::sync::{mpsc, Arc};

use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use tokiobench::params;
use tokiobench::rt;
use tokiobench::{split, split::SplitType};

type BenchFn = fn(&[usize], tx: SyncSender<()>, rem: Arc<AtomicUsize>);

#[inline]
fn work(nspawns: &[usize], tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    for nspawn in nspawns.iter().cloned() {
        let rem = rem.clone();
        let tx = tx.clone();

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
}

fn workload(bench_fn: BenchFn, st: SplitType, reverse: bool, name: &str, c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1000);
    let rem: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    let mut group = c.benchmark_group(name);

    for (nspawn, nworkers, nsplit) in iproduct!(params::NSPAWN, params::NWORKERS, params::NSPLIT) {
        group.throughput(Throughput::Elements(nspawn as u64));
        let mut workload = split::split(st, nspawn, nsplit);

        if reverse {
            workload.reverse();
        };

        let rt = rt::new(nworkers);

        group.bench_with_input(
            format!("nspawn({nspawn})/nwork({nworkers})/nsplit({nsplit}, {st})/rev({reverse})"),
            &(nspawn, nworkers),
            |b, &(nspawn, _)| {
                b.iter(|| {
                    let tx = tx.clone();
                    let rem = rem.clone();

                    rem.store(nspawn, Relaxed);
                    rt.block_on(async {
                        bench_fn(workload.as_ref(), tx, rem);

                        rx.recv().unwrap();
                    });
                });
            },
        );
    }
}

fn spawn_workload_eq(c: &mut Criterion) {
    workload(work, SplitType::Eq, false, "workload", c);
}

fn spawn_workload_gradient(c: &mut Criterion) {
    workload(work, SplitType::Gradient, false, "workload", c);
}

fn spawn_workload_reverse(c: &mut Criterion) {
    workload(work, SplitType::Gradient, true, "workload", c);
}

criterion_group!(
    spawn_benches,
    spawn_workload_eq,
    spawn_workload_gradient,
    spawn_workload_reverse
);

criterion_main!(spawn_benches);
