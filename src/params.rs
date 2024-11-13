#[cfg(feature = "full")]
pub const NWORKERS: [usize; 7] = [1, 2, 4, 6, 8, 10, 12];
#[cfg(not(any(feature = "full", feature = "maxvalonly")))]
pub const NWORKERS: [usize; 6] = [1, 2, 4, 8, 10, 12];
#[cfg(feature = "maxvalonly")]
pub const NWORKERS: [usize; 1] = [12];

#[cfg(feature = "full")]
pub const NSPAWN: [usize; 6] = [100, 1000, 10000, 100000, 1000000, 10000000];
#[cfg(not(any(feature = "full", feature = "maxvalonly")))]
pub const NSPAWN: [usize; 1] = [100000];
#[cfg(feature = "maxvalonly")]
pub const NSPAWN: [usize; 1] = [10000000];

#[cfg(feature = "full")]
pub const NSPLIT: [usize; 6] = [1, 2, 4, 6, 8, 10];
#[cfg(not(any(feature = "full", feature = "maxvalonly")))]
pub const NSPLIT: [usize; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
#[cfg(feature = "maxvalonly")]
pub const NSPLIT: [usize; 1] = [10];

pub const YIEDL_BOUND: usize = 10;
