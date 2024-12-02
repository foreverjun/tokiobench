use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use tokiobench::params;
use tokiobench::rt;
use tokiobench::work;

fn bench(name: &str, func: work::Work, c: &mut Criterion) {
    let mut group = c.benchmark_group(name);

    for (nspawn, nworkers) in iproduct!(params::NS_SPAWN_GLOBAL, params::NS_WORKERS) {
        group.throughput(Throughput::Elements(nspawn as u64));

        let rt = rt::new(nworkers);
        let mut handles = Vec::with_capacity(nspawn);

        group.bench_with_input(
            format!("nspawn({nspawn})/nwork({nworkers})"),
            &(nspawn, nworkers),
            |b, &(_, nspawn)| {
                b.iter(|| {
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

fn remote(c: &mut Criterion) {
    bench("remote", work::nothing, c);
}

fn remote_float_max(c: &mut Criterion) {
    bench("remote_float_max", work::float_max, c);
}

fn remote_int_max(c: &mut Criterion) {
    bench("remote_int_max", work::int_max, c);
}

criterion_group!(benches, remote, remote_float_max, remote_int_max,);

criterion_main!(benches);
