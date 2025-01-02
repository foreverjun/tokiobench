use tokio::runtime::{self, Runtime};

pub fn new(workers: usize, bloking: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .max_blocking_threads(bloking)
        .worker_threads(workers)
        .build()
        .unwrap()
}

pub fn new_no_lifo(workers: usize, bloking: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .max_blocking_threads(bloking)
        .worker_threads(workers)
        .disable_lifo_slot()
        .build()
        .unwrap()
}
