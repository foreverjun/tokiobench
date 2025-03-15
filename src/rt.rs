use std::time::Duration;

pub fn new_ref(nworker: usize, bloking: usize) -> tokio_ref::runtime::Runtime {
    tokio_ref::runtime::Builder::new_multi_thread()
        .max_blocking_threads(bloking)
        .worker_threads(nworker)
        .thread_keep_alive(Duration::from_secs(10_000))
        .build()
        .unwrap()
}

pub fn new_shard(nworker: usize, groups: usize, bloking: usize) -> tokio_shard::runtime::Runtime {
    tokio_shard::runtime::Builder::new_multi_thread()
        .max_blocking_threads(bloking)
        .worker_threads(nworker)
        .worker_groups(groups)
        .thread_keep_alive(Duration::from_secs(10_000))
        .build()
        .unwrap()
}
