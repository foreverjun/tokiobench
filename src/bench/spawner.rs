use std::hint::black_box;
use std::sync::mpsc::SyncSender;

pub mod reference {
    use super::*;

    use futures::future;
    use tokio_ref as tokio;

    async fn spawn_local(nspawn: usize) {
        future::join_all((0..black_box(nspawn)).map(|_| tokio::spawn(black_box(async {})))).await;
    }

    pub fn run(nspawner: usize, nspawn: usize, tx: SyncSender<()>) {
        tokio::spawn(async move {
            future::join_all((0..nspawner).map(|_| tokio::spawn(black_box(spawn_local(nspawn)))))
                .await;

            tx.send(()).unwrap()
        });
    }
}

pub mod id {
    use super::*;

    use futures::future;
    use tokio_id as tokio;

    async fn spawn_local(group: tokio::SpawnGroup, nspawn: usize) {
        let handles = std::iter::repeat(&group)
            .take(nspawn)
            .map(|group| group.spawn(black_box(async {})));
        future::join_all(handles).await;
    }

    pub fn run(nspawner: usize, nspawn: usize, tx: SyncSender<()>) {
        tokio::spawn(async move {
            future::join_all(
                (0..nspawner)
                    .map(|_| tokio::group())
                    .map(|group| tokio::spawn(spawn_local(group, nspawn))),
            )
            .await;

            tx.send(()).unwrap()
        });
    }
}
