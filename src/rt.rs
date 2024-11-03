use tokio::runtime::{self, Runtime};

pub fn new(workers: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(workers)
        .enable_all()
        .build()
        .unwrap()
}
