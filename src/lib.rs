
mod human_count;
mod human_duration;
mod human_throughput;
mod metric_prefix;
mod repr;
mod utils;

pub use human_count::{HumanCount, HumanCountData, ReprCount};
pub use metric_prefix::System;
pub use repr::Sep;
use std::borrow::Cow;

const BYTES: &str = "B";

macro_rules! impl_trait {
    ($($t:ty),+) => {$(
        impl HumanCount for $t {
            #[inline]
            fn human_count_with<'a>(
                self,
                unit: impl Into<Cow<'a, str>>,
                repr: impl Into<Option<ReprCount>>
            ) -> HumanCountData<'a> {
                HumanCountData {
                    val: self as f64,
                    unit:  unit.into(),
                    repr:  repr.into(),
                }
            }
        }
    )+}
}
impl_trait!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);

mod sealed {
    use std::time::Duration;

    pub trait Sealed {}
    macro_rules! impl_sealed {
        ($($t:ty),+) => {
            $(impl Sealed for $t {})+
        }
    }
    impl_sealed! {u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize, Duration}
}
