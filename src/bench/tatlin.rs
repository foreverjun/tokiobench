use std::sync::mpsc::SyncSender;

use futures::future;
use std::sync::Arc;

async fn task_type_1(nspawn: usize) {
    let data = Arc::new(vec![1u8; 1_000_000]);
    future::join_all(
        (0..nspawn)
            .into_iter()
            .map(|_| tokio::spawn(task_type_2(data.clone()))),
    )
    .await;
}

async fn task_type_2(data: Arc<Vec<u8>>) {
    drop(data);
}

async fn spawn_tasks(nspawner: usize, nspawn: usize, tx: SyncSender<()>) {
    future::join_all(
        (0..nspawner)
            .into_iter()
            .map(|_| tokio::spawn(task_type_1(nspawn))),
    )
    .await;

    tx.send(()).unwrap()
}

pub fn run(nspawner: usize, nspawn: usize, tx: SyncSender<()>) {
    tokio::spawn(spawn_tasks(nspawner, nspawn, tx));
}
