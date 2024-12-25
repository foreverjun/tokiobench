use std::sync::mpsc;

use itertools::iproduct;
use std::time::Duration;

use tokio_metrics as tms;
use tokiobench::bench::tatlin;
use tokiobench::metrics;
use tokiobench::params::metrics as m;
use tokiobench::path::metrics as mpath;
use tokiobench::rt;
use tokiobench::watcher;

const NUM_THREADS: usize = 12;
const NUM_WARMUP: usize = 5;

fn run_sampling(name: &str, nspawn: usize, nspawner: usize) {
    let mut leaf_handles = (0..nspawner)
        .map(|_| Vec::with_capacity(nspawn))
        .collect::<Vec<_>>();
    let mut root_handles = Vec::with_capacity(nspawner);
    let mut metrics_results = Vec::with_capacity(m::CHAN_SIZE);

    let (rt_tx, rt_rx) = mpsc::sync_channel(1);

    for niter in 0..m::N_ITER {
        metrics_results = {
            let (m_stop_tx, m_stop_rx) = mpsc::sync_channel(1);

            // create rutime, enter runtime context
            let rt = rt::new(NUM_THREADS);
            let _guard = rt.enter();

            // warmup iterations
            for _ in 0..NUM_WARMUP {
                let rt_tx = rt_tx.clone();
                tatlin::for_await_ch(nspawner, nspawn, rt_tx, root_handles, leaf_handles);
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

                tatlin::for_await_ch(nspawner, nspawn, rt_tx, root_handles, leaf_handles);
                (root_handles, leaf_handles) = rt_rx.recv().unwrap();

                m_stop_tx.send(()).unwrap();
                metrics_handler.join().unwrap()
            }
        };

        let prefix = mpath::mk_prefix(&format!(
            "sampling({name})_nspawn({nspawn})_nspawner({nspawner})"
        ));
        mpath::store_vec(&prefix, &format!("iter({niter}).csv"), &metrics_results);
        metrics_results.clear()
    }
}

fn _run_total(name: &str, nspawn: usize, nspawner: usize) {
    let mut leaf_handles = (0..nspawner)
        .map(|_| Vec::with_capacity(nspawn))
        .collect::<Vec<_>>();
    let mut root_handles = Vec::with_capacity(nspawner);
    let rt = rt::new(NUM_THREADS);

    for _ in 0..m::N_ITER {
        let (rt_tx, rt_rx) = mpsc::sync_channel(1);

        {
            let _guard = rt.enter();
            tatlin::for_await_ch(nspawner, nspawn, rt_tx, root_handles, leaf_handles);
            (root_handles, leaf_handles) = rt_rx.recv().unwrap();
        }
    }

    let metrics = metrics::total(rt);

    let prefix = mpath::mk_prefix(&format!(
        "total({})_nspawn({})_nspawner({})",
        name, nspawn, nspawner
    ));
    mpath::store_vec(&prefix, "total.csv", &[metrics]);
}

fn main() -> () {
    for (nspawn, nspawner) in iproduct!(5000..=5000, 1..10) {
        run_sampling("tatlin", nspawn, nspawner);
    }
}
