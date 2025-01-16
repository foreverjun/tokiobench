use std::sync::mpsc::SyncSender;

use tokio::task::JoinHandle;

async fn task() {
    for _ in 0..100 {
        std::hint::black_box(tokio::task::yield_now().await);
    }
}

pub type RootHandles = Vec<JoinHandle<Vec<JoinHandle<()>>>>;
pub type LeafHandles = Vec<Vec<JoinHandle<()>>>;

pub type Fn =
    fn(usize, usize, SyncSender<(RootHandles, LeafHandles)>, RootHandles, LeafHandles) -> ();

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
        handles.push(tokio::spawn(task()))
    }

    handles
}

pub fn mk_handles(nspawner: usize, nspawn: usize) -> (RootHandles, LeafHandles) {
    let leaf_handles = (0..nspawner)
        .map(|_| Vec::with_capacity(nspawn))
        .collect::<Vec<_>>();
    let root_handles = Vec::with_capacity(nspawner);

    (root_handles, leaf_handles)
}

pub fn run(
    _nspawner: usize,
    _nspawn: usize,
    tx: SyncSender<(RootHandles, LeafHandles)>,
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

pub mod origin {
    use super::*;
    use std::sync::Arc;

    async fn task_type_1(n: usize, mut handles: LeafHandle) -> LeafHandle {
        assert_eq!(n, handles.capacity());
        assert!(handles.is_empty());

        let data = Arc::new(vec![1u8; 1_000_000]);

        for _ in 0..handles.capacity() {
            handles.push(tokio::spawn(task_type_2(data.clone())));
        }

        for handle in handles.drain(..) {
            handle.await.unwrap();
        }

        handles
    }

    async fn task_type_2(data: Arc<Vec<u8>>) {
        drop(data);
    }

    type LeafHandle = Vec<JoinHandle<()>>;

    pub fn run_origin(
        _nspawner: usize,
        _nspawn: usize,
        tx: SyncSender<(RootHandles, LeafHandles)>,
        mut root_handles: RootHandles,
        mut leaf_handles: LeafHandles,
    ) {
        #[cfg(feature = "check")]
        _precond_assert(_nspawn, _nspawner, &leaf_handles, &root_handles);

        tokio::spawn(async move {
            for leaf_handle in leaf_handles.drain(..) {
                root_handles.push(tokio::spawn(task_type_1(_nspawn, leaf_handle)));
            }

            for leaf_handle in root_handles.drain(..) {
                leaf_handles.push(leaf_handle.await.unwrap());
            }

            tx.send((root_handles, leaf_handles)).unwrap();
        });
    }
}
