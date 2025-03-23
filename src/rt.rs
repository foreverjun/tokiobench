pub fn new_ref(nworker: usize, bloking: usize) -> tokio_ref::runtime::Runtime {
    tokio_ref::runtime::Builder::new_multi_thread()
        .max_blocking_threads(bloking)
        .worker_threads(nworker)
        .build()
        .unwrap()
}

pub fn new_shard(nworker: usize, ngroup: usize, nbloking: usize) -> tokio_shard::runtime::Runtime {
    tokio_shard::runtime::Builder::new_multi_thread()
        .max_blocking_threads(nbloking)
        .worker_threads(nworker)
        .worker_groups(ngroup)
        .build()
        .unwrap()
}
