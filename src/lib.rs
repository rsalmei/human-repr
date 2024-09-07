#![doc = include_str!("../README.md")]

mod human_count;
mod metric_prefix;
mod repr_base;
mod utils;

pub use human_count::{HumanCount, HumanCountData, HumanCountRepr, IntoHumanCountRepr};
pub use repr_base::{Precision, Split, System};
