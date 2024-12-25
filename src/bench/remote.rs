use cfg_if::cfg_if;

use std::sync::mpsc::SyncSender;

use tokio::task::JoinHandle;

pub type Ch = fn(usize, Vec<JoinHandle<()>>, SyncSender<Vec<JoinHandle<()>>>) -> ();

/* Add new functions to maintain inteface coherence */
fn _static_asserts() {
    let _f_: Ch = for_ch;
}

pub fn for_ch(
    nspawn: usize,
    mut handles: Vec<JoinHandle<()>>,
    tx: SyncSender<Vec<JoinHandle<()>>>,
) {
    cfg_if!(if #[cfg(feature = "check")] {
        assert!(handles.is_empty());
        assert!(handles.capacity() == nspawn);
    });

    for _ in 0..nspawn {
        handles.push(tokio::spawn(async { std::hint::black_box(()) }));
    }

    tokio::spawn(async move {
        for handle in handles.drain(..) {
            handle.await.unwrap();
        }

        tx.send(handles).unwrap();
    });
}
