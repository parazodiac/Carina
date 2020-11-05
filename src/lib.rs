extern crate pretty_env_logger;
extern crate rust_htslib;

#[macro_use]
extern crate log;

pub mod barcode;
pub mod file;

pub const CB_LENGTH: usize = 16;
pub const MIL: usize = 1_000_000;
pub const TMIL: usize = 10_000_000;
pub const HMIL: usize = 100_000_000;

