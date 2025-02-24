use std::sync::mpsc::SyncSender;
use std::hint::black_box;

pub type Bench = fn(usize, usize, SyncSender<()>);

fn _static_assert() {
    let _: Bench = origin::run;
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
