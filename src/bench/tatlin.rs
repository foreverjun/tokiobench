use std::sync::mpsc::SyncSender;

use cfg_if::cfg_if;

use tokio::task::JoinHandle;

async fn task() {
    std::hint::black_box(());
}

pub type RootHandles = Vec<JoinHandle<Vec<JoinHandle<()>>>>;
pub type LeafHandles = Vec<Vec<JoinHandle<()>>>;

pub type Fn =
    fn(usize, usize, SyncSender<(RootHandles, LeafHandles)>, RootHandles, LeafHandles) -> ();

pub type LeafFn = fn(Vec<JoinHandle<()>>) -> Vec<JoinHandle<()>>;

fn _static_assert() {
    let _: Fn = ch;
    let _: LeafFn = spawn_tasks;
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

// TODO duplication. but no call by refernce
pub fn ch(
    _nspawner: usize,
    _nspawn: usize,
    tx: SyncSender<(RootHandles, LeafHandles)>,
    mut root_handles: RootHandles,
    mut leaf_handles: LeafHandles,
) {
    cfg_if!(if #[cfg(feature = "check")] {
        assert!(root_handles.is_empty());
        assert!(root_handles.capacity() == _nspawner);

        assert!(leaf_handles.iter().all(|i| i.is_empty() && i.capacity() == _nspawn));
        assert!(leaf_handles.len() == _nspawner);
    });

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

pub mod blocking {
    use super::*;

    fn _static_assert() {
        let _: Fn = ch;
    }

    // TODO duplication. but no call by refernce
    pub fn ch(
        _nspawner: usize,
        _nspawn: usize,
        tx: SyncSender<(RootHandles, LeafHandles)>,
        mut root_handles: RootHandles,
        mut leaf_handles: LeafHandles,
    ) {
        cfg_if!(if #[cfg(feature = "check")] {
            assert!(root_handles.is_empty());
            assert!(root_handles.capacity() == _nspawner);

            assert!(leaf_handles.iter().all(|i| i.is_empty() && i.capacity() == _nspawn));
            assert!(leaf_handles.len() == _nspawner);
        });

        for leaf_handle in leaf_handles.drain(..) {
            root_handles.push(tokio::task::spawn_blocking(|| spawn_tasks(leaf_handle)));
        }

        tokio::spawn(async move {
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
