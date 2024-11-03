use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use tokiobench::rt;
use tokiobench::work;

const NWORKERS: [usize; 7] = [1, 2, 4, 6, 8, 10, 12];
const NSPAWN: [usize; 6] = [100, 1000, 10000, 100000, 1000000, 10000000];

#[inline]
fn bench(name: &str, func: fn() -> (), c: &mut Criterion) {
    let mut group = c.benchmark_group(name);

    for (nspawn, nworkers) in iproduct!(NSPAWN, NWORKERS) {
        group.throughput(Throughput::Elements(nspawn as u64));

        let rt = rt::new(nworkers);
        let mut handles = Vec::with_capacity(nspawn);

        group.bench_with_input(
            format!("nspawn({nspawn})/nwork({nworkers})"),
            &(nspawn, nworkers),
            |b, &(_, nspawn)| {
                b.iter(|| {
                    // collect metrics here TODO()
                    let func = func.clone();

                    for _ in 0..nspawn {
                        handles.push(rt.spawn(async move { func() }));
                    }

                    rt.block_on(async {
                        for handle in handles.drain(..) {
                            handle.await.unwrap();
                        }
                    });
                });
            },
        );
    }
}

fn remote_rec(c: &mut Criterion) {
    bench("remote_rec", work::rec, c);
}

fn remote_stall(c: &mut Criterion) {
    bench("remote_stall", work::stall, c);
}

fn remote_rec_stall(c: &mut Criterion) {
    bench("remote_rec_stall", work::rec_stall, c);
}

fn remote_stall_rec(c: &mut Criterion) {
    bench("remote_stall_rec", work::stall_rec, c);
}

criterion_group!(
    spawn_benches,
    remote_rec,
    remote_stall,
    remote_rec_stall,
    remote_stall_rec
);

criterion_main!(spawn_benches);
