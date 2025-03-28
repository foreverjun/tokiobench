use std::sync::mpsc;
use tokiobench::rt;

fn main() {
    let rt = rt::new_shard(10, 2, 1);
    let _guard = rt.enter();
    let (tx, rx) = mpsc::sync_channel(1);

    tokiobench::bench::tatlin::sharded::run(2, 1000, tx.clone(), 0);
    tokiobench::bench::tatlin::sharded::run(2, 1000, tx.clone(), 1);
    rx.recv().unwrap();
}
