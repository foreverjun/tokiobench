use cfg_if::cfg_if;
use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use tokiobench::params as p;
use tokiobench::rt;


fn nspawn() -> Vec<usize> {
    const BOUND: usize = 10;
    const MULTIPLYER: usize = 1000;

    (1..BOUND + 1).map(|i| i * MULTIPLYER).collect()
}

fn nspawner() -> Vec<usize> {
    const BOUND: usize = 20;

    (1..BOUND + 1).collect()
}

fn workload(name: &str, c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("workload/{name}"));
    for (nspawn, nspawner) in iproduct!(nspawn(), nspawner()) {
        let rt = rt::new(p::N_WORKERS);

        let mut leaf_handles = (0..nspawner)
            .map(|_| Vec::with_capacity(nspawn))
            .collect::<Vec<_>>();
        let mut root_handles = Vec::with_capacity(nspawner);

        group.throughput(Throughput::Elements((nspawner * nspawn) as u64));
        group.bench_function(format!("nspawn({nspawn})/nspawner({nspawner})"), |b| {
            b.iter(|| {
                cfg_if!(if #[cfg(feature = "check")] {
                    assert!(root_handles.is_empty());
                    assert!(root_handles.capacity() == nspawner);

                    assert!(leaf_handles.iter().all(|i| i.is_empty()));
                    assert!(leaf_handles.iter().all(|i| i.capacity() == nspawn));
                });

                rt.block_on(async {
                    for mut leaf_handle in leaf_handles.drain(..) {
                        root_handles.push(tokio::spawn(async move {
                            for _ in 0..leaf_handle.capacity() {
                                leaf_handle.push(tokio::spawn(async {
                                    std::hint::black_box(());
                                }));
                            }

                            leaf_handle
                        }));
                    }

                    for leaf_handle in root_handles.drain(..) {
                        let mut leaf_handle = leaf_handle.await.unwrap();

                        for handle in leaf_handle.drain(..) {
                            handle.await.unwrap();
                        }

                        leaf_handles.push(leaf_handle);
                    }
                });
            });
        });
    }
    group.finish();
}

fn bench_nothing(c: &mut Criterion) {
    workload("nothing", c)
}

criterion_group!(benches, bench_nothing);

criterion_main!(benches);
