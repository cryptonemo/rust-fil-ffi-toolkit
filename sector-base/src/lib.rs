#![cfg_attr(
    feature = "cargo-clippy",
    deny(clippy_all, clippy_perf, clippy_correctness)
)]
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(type_complexity, too_many_arguments)
)]

extern crate libc;
extern crate rand;

#[cfg(test)]
extern crate tempfile;

pub mod api;
