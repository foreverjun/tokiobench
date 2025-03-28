use std::hint::black_box;
use std::sync::mpsc::SyncSender;

pub mod reference {
    use super::*;

    use futures::future;
    use std::sync::Arc;

    async fn task_type_1(nspawn: usize) {
        let data = Arc::new(black_box(vec![1u8; 1_000_000]));
        future::join_all(
            (0..black_box(nspawn))
                .map(|_| tokio_ref::spawn(black_box(task_type_2(Arc::clone(&data))))),
        )
        .await;
    }

    async fn task_type_2(data: Arc<Vec<u8>>) {
        drop(black_box(data));
    }

    pub fn run(nspawner: usize, nspawn: usize, tx: SyncSender<()>) {
        tokio_ref::spawn(async move {
            future::join_all(
                (0..black_box(nspawner)).map(|_| tokio_ref::spawn(black_box(task_type_1(nspawn)))),
            )
            .await;

            tx.send(()).unwrap()
        });
    }
}

pub mod sharded {
    use super::*;

    use futures::future;
    use std::sync::Arc;

    async fn task_type_1(nspawn: usize, group: Arc<tokio_groups::SpawnGroup>) {
        let data = Arc::new(black_box(vec![1u8; 1_000_000]));
        future::join_all(
            (0..black_box(nspawn)).map(|_| group.spawn(black_box(task_type_2(Arc::clone(&data))))),
        )
        .await;
    }

    async fn task_type_2(data: Arc<Vec<u8>>) {
        drop(black_box(data));
    }

    pub fn run(nspawner: usize, nspawn: usize, tx: SyncSender<()>, group: usize) {
        let current = Arc::new(tokio_groups::group(group));
        let group = Arc::clone(&current);
        current.spawn(async move {
            let current = Arc::clone(&group);
            future::join_all((0..black_box(nspawner)).map(|_| {
                let group = Arc::clone(&group);
                current.spawn(black_box(task_type_1(nspawn, group)))
            }))
            .await;

            tx.send(()).unwrap()
        });
    }
}

pub mod fixed {
    use super::*;

    use futures::future;
    use std::sync::Arc;
    use tokio_fixed as tokio;

    async fn task_type_1(nspawn: usize, group: usize) {
        let data = Arc::new(black_box(vec![1u8; 1_000_000]));
        future::join_all(
            (0..black_box(nspawn))
                .map(|_| tokio::spawn_into(group, black_box(task_type_2(Arc::clone(&data))))),
        )
        .await;
    }

    async fn task_type_2(data: Arc<Vec<u8>>) {
        drop(black_box(data));
    }

    pub fn run(nspawner: usize, nspawn: usize, tx: SyncSender<()>, group: usize) {
        tokio::spawn_into(group, async move {
            future::join_all(
                (0..black_box(nspawner))
                    .map(|_| tokio::spawn_into(group, black_box(task_type_1(nspawn, group)))),
            )
            .await;

            tx.send(()).unwrap()
        });
    }
}
