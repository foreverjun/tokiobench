use std::sync::mpsc;

use itertools::iproduct;

use crate::tatlin::cleaned::run;
use tokio::time::Duration;
use tokio_metrics;
use tokiobench::bench::tatlin;
use tokiobench::monitor::watcher;
use tokiobench::path::metrics as mpath;
use tokiobench::rt;

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

    for niter in 0..SAMPLING_ITER {
        let rt = rt::new(nworker, nspawner);
        let _guard = rt.enter();
        {
            let (rt_tx, rt_rx) = mpsc::sync_channel(1);

            for _ in 0..NUM_WARMUP {
                run(nspawner, nspawn, rt_tx.clone());
                rt_rx.recv().unwrap()
            }
        }

        metrics_results = {
            let (m_stop_tx, m_stop_rx) = mpsc::sync_channel(1);
            let metrics_handler = watcher::run(
                tokio_metrics::RuntimeMonitor::new(rt.handle()),
                m_stop_rx,
                Duration::from_nanos(500),
                metrics_results,
            );
            let (rt_tx, rt_rx) = mpsc::sync_channel(1);

            run(nspawner, nspawn, rt_tx.clone());
            rt_rx.recv().unwrap();

            m_stop_tx.send(()).unwrap();
            metrics_handler.join().unwrap()
        };
        let prefix = mpath::mk_path(
            &[
                "sampling",
                &format!("nworker_{nworker}"),
                &format!("nspawner_{nspawner}"),
                &format!("nspawn_{nspawn}"),
                name,
            ],
            &format!("iter_{niter}.csv"),
        );
        mpath::store_csv(&prefix, &metrics_results);
        metrics_results.clear()
    }
}

fn main() {
    let nworker = vec![16];
    let nspawner = 8..=8;
    let nspawn = vec![5000];

    for (nworker, nspawner, nspawn) in iproduct!(nworker, nspawner, nspawn) {
        run_sampling("tatlin", nworker, nspawn, nspawner);
    }
}
