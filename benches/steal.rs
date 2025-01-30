#![allow(dead_code)]

use std::sync::mpsc;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::iproduct;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokiobench::bench::tatlin;
use tokiobench::monitor::metrics::StealOpsMeasurement;

use std::sync::Mutex;
use std::sync::OnceLock;
use tokiobench::rt;

type SharedRT = Arc<Mutex<Box<Option<Runtime>>>>;

/// Make runtime global state
/// to satisfy poor criterion.rs custrom measurements interface
/// TODO()
static RT: OnceLock<SharedRT> = OnceLock::new();

fn alloc_box() -> SharedRT {
    let foi = RT.get_or_init(|| Arc::new(Mutex::new(Box::new(None))));
    Arc::clone(foi)
}

fn bench(
    name: &str,
    fun: tatlin::Bench,
    nspawn: &[usize],
    nspawner: &[usize],
    nworker: &[usize],
    c: &mut Criterion<StealOpsMeasurement>,
) {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut group = c.benchmark_group(format!("tatlin/{name}"));

    for (&nspawn, &nspawner, &nworker) in iproduct!(nspawn, nspawner, nworker) {
        let rt = rt::new(nworker, 1);
        let _gurad = rt.enter(); // entering runtime

        {
            let shared_rt = Arc::clone(RT.get().unwrap());
            let mut shared_rt = shared_rt.lock().unwrap();
            shared_rt.replace(rt);
        }

        group.throughput(Throughput::Elements((nspawn * nspawner) as u64));
        group.sampling_mode(criterion::SamplingMode::Linear);

        group.bench_function(
            format!("nworker({nworker})/nspawner({nspawner})/nspawn({nspawn})"),
            |b| {
                b.iter(|| {
                    fun(nspawner, nspawn, tx.clone());
                    rx.recv().unwrap()
                });
            },
        );
    }
    group.finish();
}

fn nworker() -> Vec<usize> {
    vec![1, 2, 4, 8, 12]
}

fn nspawner() -> Vec<usize> {
    (1..=60).collect()
}

macro_rules! benches {
    ($expression:tt) => {
        pub fn origin(c: &mut Criterion<StealOpsMeasurement>) {
            bench(
                concat!($expression, "/origin"),
                tatlin::origin::run,
                &nspawn(),
                &nspawner(),
                &nworker(),
                c,
            )
        }
    };
}

pub mod scatter {
    use super::*;

    fn nspawn() -> Vec<usize> {
        (1..=50).map(|i| i * 1000).collect()
    }

    benches! {"scatter"}
}

pub mod line {
    use super::*;

    fn nspawn() -> Vec<usize> {
        vec![5000]
    }

    benches! {"line"}
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(5))
        .with_measurement(tokiobench::monitor::metrics::StealOpsMeasurement { rt: alloc_box() });

    targets = line::origin
);

criterion_main!(benches);
