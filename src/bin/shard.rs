use std::sync::mpsc;
use tokiobench::rt;

fn main() {
    let rt = rt::new_shard(10, 4, 1);
    let _guard = rt.enter();
    let (tx, rx) = mpsc::sync_channel(1);

    tokiobench::bench::tatlin::sharded::run(16, 1000, tx);
    rx.recv().unwrap();
}
