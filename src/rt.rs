use tokio::runtime::{self, Runtime};

pub fn new(workers: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(workers)
        .build()
        .unwrap()
}

pub fn new_no_lifo(workers: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(workers)
        .disable_lifo_slot()
        .build()
        .unwrap()
}
