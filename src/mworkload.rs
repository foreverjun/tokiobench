use cfg_if::cfg_if;
use itertools::iproduct;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{mpsc, Arc};
use tokio::task::JoinHandle;
use tokio_metrics::RuntimeMetrics;
use tokiobench::params as p;
use tokiobench::params::metrics as m;
use tokiobench::path::metrics as mpath;
use tokiobench::rt;
use tokiobench::watcher;

type Handles = Vec<JoinHandle<()>>;

fn run_iter(
    nspawn: usize,
    nspawner: usize,
    mut root_handles: Vec<JoinHandle<Handles>>,
    mut leaf_handles: Vec<Handles>,
    sample_size: u64,
) -> (Vec<JoinHandle<Handles>>, Vec<Handles>, Vec<RuntimeMetrics>) {
    let rt = rt::new(p::N_WORKERS);
    let (tx, rx) = mpsc::sync_channel(1);
    let (m_tx, m_rx) = mpsc::sync_channel(m::CHAN_SIZE);
    let rem: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));

    cfg_if!(if #[cfg(feature = "check")] {
                        assert!(root_handles.is_empty());
                        assert!(root_handles.capacity() == nspawner);

                        assert!(leaf_handles.iter().all(|i| i.is_empty()));
                        assert!(leaf_handles.iter().all(|i| i.capacity() == nspawn));
                    });

    let metrics_handler = {
        let rem = Arc::clone(&rem);
        let handle = rt.handle();
        let rt_monitor = tokio_metrics::RuntimeMonitor::new(&handle);

        watcher::run(m_tx, rem, rt_monitor, sample_size)
    };


    let _guard = rt.enter();

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

    tokio::spawn(async move {
        for leaf_handle in root_handles.drain(..) {
            let mut leaf_handle = leaf_handle.await.unwrap();

            for handle in leaf_handle.drain(..) {
                handle.await.unwrap();
            }

            leaf_handles.push(leaf_handle);
        }

        tx.send((leaf_handles, root_handles)).unwrap();
    });

    (leaf_handles, root_handles) = rx.recv().unwrap();

    rem.fetch_sub(1, Relaxed);
    metrics_handler.join().unwrap();

    assert!(root_handles.is_empty());

    (root_handles, leaf_handles , m_rx.into_iter().collect::<Vec<_>>())
}


fn run_metrics(name: &str, nspawn: &[usize], nspawner: &[usize], sample_slice:u64) {
    for (&nspawn, &nspawner) in iproduct!(nspawn, nspawner) {
        let name = format!("{name}/nspawn({nspawn})_nspawner({nspawner})");
        let prefix = mpath::mk_prefix(&name);
        let mut leaf_handles = (0..nspawner)
            .map(|_| Vec::with_capacity(nspawn))
            .collect::<Vec<_>>();
        let mut root_handles = Vec::with_capacity(nspawner);

        for niter in 0..m::N_ITER {
            let output = run_iter(nspawn, nspawner, root_handles, leaf_handles, sample_slice);
            root_handles = output.0;
            leaf_handles = output.1;
            let metrics_u8 = serde_json::to_vec_pretty(&output.2).unwrap();

            let name = format!("iter_{niter}.json");
            mpath::store(&prefix, &name, &metrics_u8);
        }
    }
}

fn main() -> () {
    let nspawn: Vec<usize> = (1..=15).map(|i| i * 1000).collect();
    let nspawner: Vec<usize> = (1..=20).collect();
    run_metrics("thousands", &nspawn, &nspawner, 1);
    let nspawn: Vec<usize> = (1..=10).map(|i| i * 100_000).collect();
    run_metrics("hthousands", &nspawn, &nspawner, 10);
    let nspawn: Vec<usize> = (1..=10).map(|i| i * 1_000_000).collect();
    run_metrics("millions", &nspawn, &nspawner, 50);

}