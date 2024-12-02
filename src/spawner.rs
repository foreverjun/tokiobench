use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

use crate::{params, work};

pub type BenchFn = fn(usize, SyncSender<()>, Arc<AtomicUsize>) -> ();

// NOTE: what about add some work in producer

#[inline]
fn spawn_current_main(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>, work: fn() -> ()) {
    for _ in 0..nspawn {
        let tx = tx.clone();
        let rem = rem.clone();

        tokio::spawn(async move {
            for _ in 0..params::YIEDL_BOUND {
                std::hint::black_box(work());
                tokio::task::yield_now().await;
            }

            if 1 == rem.fetch_sub(1, Relaxed) {
                tx.send(()).unwrap();
            }
        });
    }
}

#[inline(always)]
pub fn spawn_current(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_current_main(nspawn, tx, rem, work::nothing);
}

// float
#[inline(always)]
pub fn spawn_current_min_float(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_current_main(nspawn, tx, rem, work::float_min);
}

#[inline(always)]
pub fn spawn_current_mid_float(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_current_main(nspawn, tx, rem, work::float_mid);
}

#[inline(always)]
pub fn spawn_current_max_float(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_current_main(nspawn, tx, rem, work::float_max);
}

// int
#[inline(always)]
pub fn spawn_current_min_int(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_current_main(nspawn, tx, rem, work::int_min);
}

#[inline(always)]
pub fn spawn_current_mid_int(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_current_main(nspawn, tx, rem, work::int_mid);
}

#[inline(always)]
pub fn spawn_current_max_int(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_current_main(nspawn, tx, rem, work::int_max);
}

#[inline]
fn spawn_local_main(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>, work: fn() -> ()) {
    tokio::spawn(async move {
        for _ in 0..nspawn {
            let rem = rem.clone();
            let tx = tx.clone();

            tokio::spawn(async move {
                for _ in 0..params::YIEDL_BOUND {
                    std::hint::black_box(work());
                    tokio::task::yield_now().await;
                }

                if 1 == rem.fetch_sub(1, Relaxed) {
                    tx.send(()).unwrap();
                }
            });
        }
    });
}

#[inline]
pub fn spawn_local(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_local_main(nspawn, tx, rem, work::nothing);
}

// float
#[inline(always)]
pub fn spawn_local_min_float(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_local_main(nspawn, tx, rem, work::float_min);
}

#[inline(always)]
pub fn spawn_local_mid_float(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_local_main(nspawn, tx, rem, work::float_mid);
}

#[inline(always)]
pub fn spawn_local_max_float(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_local_main(nspawn, tx, rem, work::float_max);
}

// int
#[inline(always)]
pub fn spawn_local_min_int(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_local_main(nspawn, tx, rem, work::int_min);
}

#[inline(always)]
pub fn spawn_local_mid_int(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_local_main(nspawn, tx, rem, work::int_mid);
}

#[inline(always)]
pub fn spawn_local_max_int(nspawn: usize, tx: SyncSender<()>, rem: Arc<AtomicUsize>) {
    spawn_local_main(nspawn, tx, rem, work::int_max);
}
