#![cfg_attr(feature = "nightly", feature(core_intrinsics))]

mod map;

pub use map::*;
pub(crate) mod exhaust;
