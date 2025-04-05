use std::sync::mpsc;
use tokiobench::rt;

fn main() {
    let rt = rt::new_ref(12, 1);
    let _guard = rt.enter();
    let (tx, rx) = mpsc::sync_channel(1);

    tokiobench::bench::tatlin::reference::run(4, 1000, tx);
    rx.recv().unwrap();
}
