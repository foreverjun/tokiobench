use tokio::runtime::{self, Runtime};
use std::time::Duration;

pub fn new(workers: usize, bloking: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .max_blocking_threads(bloking)
        .worker_threads(workers)
        .thread_keep_alive(Duration::from_secs(10_000))
        .build()
        .unwrap()
}
