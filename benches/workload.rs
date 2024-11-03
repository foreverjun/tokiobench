use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;
use std::sync::{mpsc, Arc};

use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use tokiobench::rt;
use tokiobench::{split, split::SplitType};

const NWORKERS: [usize; 7] = [1, 2, 4, 6, 8, 10, 12];
const NSPAWN: [usize; 6] = [100, 1000, 10000, 100000, 1000000, 10000000];
const NSPLIT: [usize; 7] = [1, 2, 4, 6, 8, 10, 12];

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

#[inline]
fn workload(bench_fn: BenchFn, st: SplitType, reverse: bool, name: &str, c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1000);
    let rem: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    let mut group = c.benchmark_group(name);

    for (nspawn, nworkers, nsplit) in iproduct!(NSPAWN, NWORKERS, NSPLIT) {
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
                    // collect metrics here TODO()
                    rt.block_on(async {
                        bench_fn(workload.as_ref(), tx, rem);

                        rx.recv().unwrap();
                    });
                });
            },
        );
    }
}

fn rt_spawn_workload_eq(c: &mut Criterion) {
    workload(work, SplitType::Eq, false, "workload", c);
}

fn rt_spawn_workload_gradient(c: &mut Criterion) {
    workload(work, SplitType::Gradient, false, "workload", c);
}

// TODO()
// fn rt_spawn_workload_reverse(c: &mut Criterion) {
//     workload(work, true, "workload_reversed", c);
// }

criterion_group!(
    spawn_benches,
    //rt_spawn_workload_eq,
    rt_spawn_workload_gradient
);

criterion_main!(spawn_benches);
