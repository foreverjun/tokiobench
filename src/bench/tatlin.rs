use std::sync::mpsc::SyncSender;

use futures::future;
use std::hint::black_box;

pub type Bench = fn(usize, usize, SyncSender<()>);

pub mod buffered {
    // todo!
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
        black_box(drop(black_box(data)));
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
