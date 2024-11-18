use std::time::{Duration, Instant};

const STALL_DUR: Duration = Duration::from_micros(10);

pub type Type = fn() -> ();

fn stall_inner(func: impl Fn() -> ()) {
    let now = Instant::now();

    while now.elapsed() < STALL_DUR {
        std::hint::black_box(func());

        std::thread::yield_now();
    }
}

pub fn stall() {
    stall_inner(|| {});
}

pub fn stall_rec() {
    stall_inner(rec);
}

fn rec_inner(func: impl Fn() -> ()) {
    const LEN: usize = 10;

    fn run(func: impl Fn() -> (), count: usize) {
        if count <= 0 {
            return;
        }

        std::hint::black_box(func());
        std::hint::black_box(run(func, count - 1));
    }

    run(func, LEN)
}

pub fn rec() {
    rec_inner(|| {});
}

pub fn rec_stall() {
    rec_inner(stall);
}

pub fn nothing() {
    std::hint::black_box(())
}
