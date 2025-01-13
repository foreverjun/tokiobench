use std::sync::mpsc;

use itertools::iproduct;
use tokiobench::bench::tatlin::mk_handles;

use tokiobench::bench::tatlin;
use tokiobench::metrics;
use tokiobench::path::metrics as mpath;
use tokiobench::rt;

const NUM_WARMUP: usize = 30;

mod log {
    pub fn starting(stype: &str, name: &str, nworker: usize, nspawn: usize, nspawner: usize) {
        println!(
            "starting {stype} for {name} with nworker: {nworker}, nspawn: {nspawn}, nspawner: {nspawner}"
        );
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
    let nspawner = 1..=50;
    let nspawn = vec![5000];

    for (nworker, nspawner, nspawn) in iproduct!(nworker, nspawner, nspawn) {
        run_total("tatlin", nworker, nspawn, nspawner);
    }
}
