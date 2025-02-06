use std::sync::mpsc::SyncSender;

use futures::future;
use std::hint::black_box;

pub type Bench = fn(usize, usize, SyncSender<()>);

pub mod buffered {
    use super::*;
    use tokio::task::JoinHandle;

    pub fn mk_hs(nspawner: usize, nspawn: usize) -> (RootHandles, LeafHandles) {
        let leaf_handles = (0..nspawner)
            .map(|_| Vec::with_capacity(nspawn))
            .collect::<Vec<_>>();
        let root_handles = Vec::with_capacity(nspawner);

        (root_handles, leaf_handles)
    }

    pub type RootHandles = Vec<JoinHandle<Vec<JoinHandle<()>>>>;
    pub type LeafHandles = Vec<Vec<JoinHandle<()>>>;
    pub type SyncerSender = SyncSender<(RootHandles, LeafHandles)>;

    pub type Fn = fn(usize, usize, SyncerSender, RootHandles, LeafHandles) -> ();

    pub type LeafFn = fn(Vec<JoinHandle<()>>) -> Vec<JoinHandle<()>>;

    fn _static_assert() {
        let _: Fn = run;
        let _: LeafFn = spawn_tasks;
    }

    fn _precond_assert(
        nspawn: usize,
        nspawner: usize,
        leaf_handles: &LeafHandles,
        root_handles: &RootHandles,
    ) {
        assert!(root_handles.is_empty());
        assert!(root_handles.capacity() == nspawner);

        assert!(leaf_handles
            .iter()
            .all(|i| i.is_empty() && i.capacity() == nspawn));
        assert!(leaf_handles.len() == nspawner);
    }

    fn spawn_tasks(mut handles: Vec<JoinHandle<()>>) -> Vec<JoinHandle<()>> {
        for _ in 0..handles.capacity() {
            handles.push(tokio::spawn(async { std::hint::black_box(()) }))
        }

        handles
    }

    pub fn run(
        _nspawner: usize,
        _nspawn: usize,
        tx: SyncerSender,
        mut root_handles: RootHandles,
        mut leaf_handles: LeafHandles,
    ) {
        #[cfg(feature = "check")]
        _precond_assert(_nspawn, _nspawner, &leaf_handles, &root_handles);

        tokio::spawn(async move {
            for leaf_handle in leaf_handles.drain(..) {
                root_handles.push(tokio::spawn(async move { spawn_tasks(leaf_handle) }));
            }

            for leaf_handle in root_handles.drain(..) {
                let mut leaf_handle = leaf_handle.await.unwrap();

                for leaf in leaf_handle.drain(..) {
                    leaf.await.unwrap();
                }

                leaf_handles.push(leaf_handle)
            }

            tx.send((root_handles, leaf_handles)).unwrap()
        });
    }
}

pub mod cleaned {
    use super::*;

    async fn task() {
        black_box(());
    }

    async fn spawn_tasks(nspawn: usize) {
        let handles = (0..nspawn).map(|_| tokio::spawn(task()));
        future::join_all(handles).await;
    }

    pub fn run(_nspawner: usize, _nspawn: usize, tx: SyncSender<()>) {
        tokio::spawn(async move {
            let hs = (0.._nspawner).map(|_| tokio::spawn(spawn_tasks(_nspawn)));
            future::join_all(hs).await;

            tx.send(()).unwrap();
        });
    }
}

pub mod origin {
    use super::*;

    use futures::future;
    use std::sync::Arc;

    async fn task_type_1(nspawn: usize) {
        let data = Arc::new(black_box(vec![1u8; 1_000_000]));
        future::join_all(
            (0..black_box(nspawn)).map(|_| tokio::spawn(black_box(task_type_2(Arc::clone(&data))))),
        )
        .await;
    }

    async fn task_type_2(data: Arc<Vec<u8>>) {
        black_box(());
        drop(black_box(data));
        black_box(());
    }

    pub fn run(nspawner: usize, nspawn: usize, tx: SyncSender<()>) {
        tokio::spawn(async move {
            future::join_all(
                (0..black_box(nspawner)).map(|_| tokio::spawn(black_box(task_type_1(nspawn)))),
            )
            .await;

            tx.send(()).unwrap()
        });
    }
}
