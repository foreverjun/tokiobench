use std::thread;

use std::time::Duration;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use tokio_metrics::RuntimeMonitor;

use crate::params::metrics as m;

pub type MetricSyncSender = SyncSender<tokio_metrics::RuntimeMetrics>;

pub fn run(
    metric_tx: MetricSyncSender,
    mut stop_rx: Receiver<bool>,
    rt_monitor: RuntimeMonitor,
    sample_slice: u64
) -> std::thread::JoinHandle<()> {
    let thread_handle = thread::spawn(move || {
        let mut metrics_count = 0;

        for interval in rt_monitor.intervals() {
            metrics_count += 1;
            if metrics_count >= m::CHAN_SIZE {
                panic!("metrics overflow");
            }
            metric_tx.send(interval).unwrap();

            if stop_rx.try_recv().is_ok() { break; }

            thread::sleep(Duration::from_millis(sample_slice));
        }
    });

    thread_handle
}
