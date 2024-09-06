mod data;
mod repr;

use crate::utils::{sealed, BYTES};
pub use data::HumanCountData;
pub use repr::{HumanCountRepr, IntoHumanCountRepr};

/// Generate beautiful human-friendly counts.
pub trait HumanCount: sealed::Sealed + Sized {
    /// Generate a beautiful human-friendly count with automatic prefixes.
    ///
    /// It will use the default system, precision, and Split.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.22M", 4221432u32.human_count());
    /// ```
    /// "#
    #[inline]
    fn human_count(self) -> HumanCountData {
        self.human_count_as(())
    }

    /// Generate a beautiful human-friendly count with automatic prefixes and the `"B"` (bytes) unit.
    ///
    /// It will use the default system, precision, and Split.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.22MB", 4221432u32.human_count_bytes());
    /// ```
    #[inline]
    fn human_count_bytes(self) -> HumanCountData {
        self.human_count_as(BYTES)
    }

    /// Generate a beautiful human-friendly count with automatic prefixes and a custom representation.
    ///
    /// Just send an `&str` unit, a [System](crate::System), a [Precision](crate::Precision) (or u8
    /// for fixed precision), or a [Split](crate::Split) (or a bool), or even a tuple of them all,
    /// in any order!
    ///
    /// ```
    /// use human_repr::{HumanCount, Precision, Split, System};
    /// assert_eq!("1.23M🦀", 1234567u32.human_count_as("🦀"));
    /// assert_eq!("1.18Mi", 1234567u32.human_count_as(System::IEC));
    /// assert_eq!("1.2346M", 1234567u32.human_count_as(4));
    /// assert_eq!("1.23 M", 1234567u32.human_count_as(Split::Prefix));
    /// assert_eq!("1.234567Mtests", 1234567u32.human_count_as((Precision::Full, "tests")));
    /// assert_eq!("1.2M °C", 1234567u32.human_count_as(("°C", 1, Split::Unit)));
    /// assert_eq!("1.18MHz", 1234567u32.human_count_as((2, System::SI2, "Hz", false)));
    /// ```
    fn human_count_as(self, repr: impl IntoHumanCountRepr) -> HumanCountData;
}

macro_rules! impl_trait {
    ($($t:ty),+) => {$(
        impl HumanCount for $t {
            #[inline]
            fn human_count_as(self, repr: impl IntoHumanCountRepr) -> HumanCountData{
                HumanCountData {
                    val: self as f64,
                    repr:  repr.into_repr(),
                }
            }
        }
    )+}
}
impl_trait!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);
