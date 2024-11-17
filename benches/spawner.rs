use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{mpsc, Arc};

use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use tokiobench::params;
use tokiobench::rt;
use tokiobench::spawner as sp;

fn bench_count_down(bench_fn: sp::BenchFn, name: &str, c: &mut Criterion) {
    let (tx, rx) = mpsc::sync_channel(1000);
    let rem: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    let mut group = c.benchmark_group(name);

    for (nspawn, nworkers) in iproduct!(params::NS_SPAWN_LOCAL, params::NS_WORKERS) {
        let rt = rt::new(nworkers);

        group.throughput(Throughput::Elements(nspawn as u64));
        group.bench_with_input(
            format!("nspawn({nspawn})/nwork({nworkers})"),
            &nspawn,
            |b, &nspawn| {
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
    bench_count_down(sp::spawn_current_recstall, "spawn_current", c)
}

fn spawn_many_from_local_bench(c: &mut Criterion) {
    bench_count_down(sp::spawn_local_recstall, "spawn_local", c);
}

criterion_group!(
    spawn_benches,
    spawn_many_from_current_bench,
    spawn_many_from_local_bench
);

criterion_main!(spawn_benches);
