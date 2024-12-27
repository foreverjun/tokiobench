use std::sync::mpsc;
use std::time::Duration;

use tokio_metrics as tms;
use tokiobench::bench::remote;
use tokiobench::params::metrics as m;
use tokiobench::path::metrics as mpath;
use tokiobench::rt;
use tokiobench::watcher;

const NUM_WARMUP: usize = 5;

fn run_sampling(name: &str, nspawn: usize, nworker: usize) {
    let mut handles = Vec::with_capacity(nspawn);
    let mut metrics_results = Vec::with_capacity(m::CHAN_SIZE);

    let (rt_tx, rt_rx) = mpsc::sync_channel(1);

    for niter in 0..m::N_ITER {
        metrics_results = {
            let (m_stop_tx, m_stop_rx) = mpsc::sync_channel(1);

            // create rutime, enter runtime context
            let rt = rt::new(nworker);
            let _guard = rt.enter();

            // warmup iterations
            for _ in 0..NUM_WARMUP {
                let rt_tx = rt_tx.clone();
                remote::for_ch(nspawn, handles, rt_tx);
                (handles) = rt_rx.recv().unwrap();
            }

            {
                let metrics_handler = watcher::run(
                    tms::RuntimeMonitor::new(rt.handle()),
                    m_stop_rx,
                    Duration::from_nanos(500),
                    metrics_results,
                );

                let rt_tx = rt_tx.clone();
                remote::for_ch(nspawn, handles, rt_tx);
                (handles) = rt_rx.recv().unwrap();

                m_stop_tx.send(()).unwrap();
                metrics_handler.join().unwrap()
            }
        };

        let prefix = mpath::mk_prefix(&format!(
            "sampling({name})_nspawn({nspawn})_nworker({nworker})"
        ));
        mpath::store_vec(&prefix, &format!("iter({niter}).csv"), &metrics_results);
        metrics_results.clear()
    }
}

fn main() -> () {
    const NSPAWN: usize = 1000_000;

    for nworker in 2..24 {
        run_sampling("million", NSPAWN, nworker);
    }
}
