use std::time::Duration;
use tokio::runtime::{self, Runtime};

fn cores() -> impl Iterator<Item = usize> {
    const CORE_BOUND: usize = 48;

    (0..CORE_BOUND).step_by(2)
}

pub fn new(nworker: usize, bloking: usize) -> Runtime {
    let cores = cores().collect::<Vec<usize>>();
    assert!(nworker <= cores.len());

    runtime::Builder::new_multi_thread()
        .max_blocking_threads(bloking)
        .worker_threads(nworker)
        .on_thread_start(move || {
            affinity::set_thread_affinity(&cores)
                .expect("affinity setting...");
        })
        .thread_keep_alive(Duration::from_secs(10_000))
        .build()
        .unwrap()
}
