use std::sync::mpsc;

use itertools::iproduct;
use std::time::Duration;
use tokiobench::bench::tatlin::mk_handles;

use tokio_metrics as tms;
use tokiobench::bench::tatlin;
use tokiobench::metrics;
use tokiobench::path::metrics as mpath;
use tokiobench::rt;
use tokiobench::watcher;

const NUM_WARMUP: usize = 30;

const SAMPLING_ITER: usize = 100;
const SAMPLING_METRICS_BOUND: usize = 1_000_000;

mod log {
    pub fn starting(stype: &str, name: &str, nworker: usize, nspawn: usize, nspawner: usize) {
        println!(
            "starting {stype} for {name} with nworker: {nworker}, nspawn: {nspawn}, nspawner: {nspawner}"
        );
    }
}

fn run_sampling(name: &str, nworker: usize, nspawn: usize, nspawner: usize) {
    log::starting("sampling", name, nworker, nspawn, nspawner);

    let mut metrics_results = Vec::with_capacity(SAMPLING_METRICS_BOUND);
    let (mut root_handles, mut leaf_handles) = mk_handles(nspawner, nspawn);

    let (rt_tx, rt_rx) = mpsc::sync_channel(1);

    for niter in 0..SAMPLING_ITER {
        metrics_results = {
            let (m_stop_tx, m_stop_rx) = mpsc::sync_channel(1);

            // create rutime, enter runtime context
            let rt = rt::new(nworker, nspawner);
            let _guard = rt.enter();

            // warmup iterations
            for _ in 0..NUM_WARMUP {
                let rt_tx = rt_tx.clone();
                tatlin::run_blocking(nspawner, nspawn, rt_tx, root_handles, leaf_handles);
                (root_handles, leaf_handles) = rt_rx.recv().unwrap();
            }

            {
                let metrics_handler = watcher::run(
                    tms::RuntimeMonitor::new(rt.handle()),
                    m_stop_rx,
                    Duration::from_nanos(500),
                    metrics_results,
                );

                let rt_tx = rt_tx.clone();

                tatlin::run_blocking(nspawner, nspawn, rt_tx, root_handles, leaf_handles);
                (root_handles, leaf_handles) = rt_rx.recv().unwrap();

                m_stop_tx.send(()).unwrap();
                metrics_handler.join().unwrap()
            }
        };

        let prefix = mpath::mk_path(
            &[
                "sampling",
                &format!("nworker:{nworker}"),
                &format!("nspawner:{nspawner}"),
                &format!("nspawn:{nspawn}"),
                name,
            ],
            &format!("iter:{niter}.csv"),
        );
        mpath::store_csv(&prefix, &metrics_results);
        metrics_results.clear()
    }
}

const TOTAL_ITERS: usize = 100;

fn run_total(name: &str, nworker: usize, nspawn: usize, nspawner: usize) {
    log::starting("total", name, nworker, nspawn, nspawner);

    let (mut root_handles, mut leaf_handles) = mk_handles(nspawner, nspawn);

    {
        // warmup
        let (rt_tx, rt_rx) = mpsc::sync_channel(1);

        for _ in 0..NUM_WARMUP {
            let rt = rt::new(nworker, nspawner);

            let _guard = rt.enter();
            tatlin::run_blocking(nspawner, nspawn, rt_tx.clone(), root_handles, leaf_handles);
            (root_handles, leaf_handles) = rt_rx.recv().unwrap();
        }
    }

    let metrics = {
        // execution
        let rt = rt::new(nworker, nspawner);
        let (rt_tx, rt_rx) = mpsc::sync_channel(1);

        for _ in 0..TOTAL_ITERS {
            let _guard = rt.enter();
            tatlin::run_blocking(nspawner, nspawn, rt_tx.clone(), root_handles, leaf_handles);
            (root_handles, leaf_handles) = rt_rx.recv().unwrap();
        }

        metrics::total(rt)
    };

    let prefix = mpath::mk_path(
        &[
            "total",
            &format!("nworker:{nworker}"),
            &format!("nspawner:{nspawner}"),
            &format!("nspawn:{nspawn}"),
            name,
        ],
        "total.json",
    );
    mpath::store_json(&prefix, &metrics);
}

fn main() {
    let nworker = vec![1, 2, 4, 8, 12, 16, 24];

    for (nworker, nspawn, nspawner) in iproduct!(nworker, 5000..=5000, 1..=20) {
        run_sampling("tatlin", nworker, nspawn, nspawner);
        run_total("tatlin", nworker, nspawn, nspawner);
    }
}
