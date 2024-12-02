use std::hint::black_box;

const STRANGE_FLOAT: f64 = 3.123456;
const STRANGE_INT: usize = 13;

pub type Work = fn() -> ();

pub enum Op {
    Mul,
    Add,
    MulAdd,
}

#[inline(always)]
fn float(count: usize, op: Op) {
    let mut n: f64 = 0.0;
    for _ in 0..count {
        match op {
            Op::Add => n += black_box(STRANGE_FLOAT),
            Op::Mul => n *= black_box(STRANGE_FLOAT),
            Op::MulAdd => n = n.mul_add(black_box(STRANGE_FLOAT), black_box(STRANGE_FLOAT)),
        }
    }
    black_box(n);
}

#[inline(always)]
fn int(count: usize, op: Op) {
    let mut n: usize = 0;

    for _ in 0..count {
        match op {
            Op::Add => n = n.wrapping_add(black_box(STRANGE_INT)),
            Op::Mul => n = n.wrapping_mul(black_box(STRANGE_INT)),
            Op::MulAdd => {
                n = n.wrapping_mul(black_box(STRANGE_INT));
                n = n.wrapping_add(black_box(STRANGE_INT));
            }
        }
    }

    black_box(n);
}

// float
#[inline(always)]
pub fn float_min() {
    float(crate::params::work::MIN, Op::MulAdd);
}

#[inline(always)]
pub fn float_mid() {
    float(crate::params::work::MID, Op::MulAdd);
}

#[inline(always)]
pub fn float_max() {
    float(crate::params::work::MAX, Op::MulAdd);
}

// int
#[inline(always)]
pub fn int_min() {
    int(crate::params::work::MIN, Op::MulAdd);
}

#[inline(always)]
pub fn int_mid() {
    int(crate::params::work::MID, Op::MulAdd);
}

#[inline(always)]
pub fn int_max() {
    int(crate::params::work::MAX, Op::MulAdd);
}

#[inline(always)]
pub fn nothing() {}
