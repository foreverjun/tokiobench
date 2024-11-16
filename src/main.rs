use std::io::Write;
use std::{fs, thread};

use std::time::Duration;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{mpsc, Arc};

use tokiobench::rt;
use tokiobench::work;

use std::fs::File;
use std::path::{PathBuf, Path};

const WAIT_TIME: u64 = 500;

const N_WORKERS: usize = 10;
const N_SPAWN: usize = 100;
const YIEDL_BOUND: usize = 100;

const METRIC_CHAN_SIZE: usize = 100000;

fn metrics_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();

    path.push("target");
    path.push("metrics");

    path
}

fn mk_metrics_dir() {
    let path = metrics_path();
    println!("{:?}", path);

    if Path::exists(&path) {
        return;
    }

    fs::create_dir(path).unwrap();
}

fn store(name: &str, data: &[u8]) {
    let result_path = {
        let mut mp = metrics_path();
        mp.push(name);
        mp
    };

    let mut f = File::create(result_path).unwrap();
    f.write_all(data).unwrap();
}


fn main() -> () {
    let rt = rt::new(N_WORKERS);

    let (tx, rx) = mpsc::sync_channel(1);
    let (m_tx, m_rx) = mpsc::sync_channel(METRIC_CHAN_SIZE);
    let rem: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(N_SPAWN));

    let metrics_handler = {
        let rem = rem.clone();
        let handle = rt.handle();
        let runtime_monitor= tokio_metrics::RuntimeMonitor::new(&handle);

        let thread_handle = thread::spawn(move || {
            let mut metrics_count = 0;

            for interval in runtime_monitor.intervals() {
                metrics_count += 1;
                if metrics_count >= METRIC_CHAN_SIZE {
                    panic!("metrics overflow");
                }
                m_tx.send(interval).unwrap();

                if rem.load(Relaxed) == 0 {
                    break;
                }

                thread::sleep(Duration::from_millis(WAIT_TIME));
            }
        });

        thread_handle
    };

    rt.block_on(async move {
        for _ in 0..N_SPAWN {
            let tx = tx.clone();
            let rem = rem.clone();

            tokio::spawn(async move {
                for _ in 0..YIEDL_BOUND {
                    tokio::task::yield_now().await;
                    work::rec_stall();
                }

                if 1 == rem.fetch_sub(1, Relaxed) {
                    tx.send(()).unwrap();
                }
            });
        }

        rx.recv().unwrap();
    });

    metrics_handler.join().unwrap();

    let result = m_rx.into_iter().collect::<Vec<_>>();
    let result = serde_json::to_vec_pretty(&result).unwrap();

    mk_metrics_dir();

    store("spawner.json",&result);
}
