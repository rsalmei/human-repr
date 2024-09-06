mod data;
mod repr;

use crate::utils::{sealed, BYTES};
pub use data::HumanCountData;
pub use repr::{HumanCountRepr, IntoHumanCountRepr};

/// Generate beautiful human-friendly counts.
pub trait HumanCount: sealed::Sealed + Sized {
    /// Generate a beautiful human-friendly count with automatic prefixes.
    #[inline]
    fn human_count(self) -> HumanCountData {
        self.human_count_as::<&'static str>(None)
    }

    /// Generate a beautiful human-friendly count with automatic prefixes and `"B"` (bytes) unit.
    #[inline]
    fn human_count_bytes(self) -> HumanCountData {
        self.human_count_as(BYTES)
    }

    /// Generate a beautiful human-friendly count with automatic prefixes and custom representations.
    ///
    /// Just send a unit, a [`System`], a [`Precision`] (or u8 for fixed precision), or a [`Separator`]
    /// (or a bool), or even a tuple of them all, in any order!
    /// ```
    /// use human_repr::{HumanCount, Precision, Separator, System};
    /// assert_eq!("1.23M🦀", 1234567u32.human_count_as("🦀"));
    /// assert_eq!("1.18Mi", 1234567u32.human_count_as(System::IEC));
    /// assert_eq!("1.2346M", 1234567u32.human_count_as(4));
    /// assert_eq!("1.23 M", 1234567u32.human_count_as(true));
    /// assert_eq!("1.235 M°C", 1234567u32.human_count_as(("°C", 3, Separator::Yes)));
    /// assert_eq!("1.234567Mtests", 1234567u32.human_count_as((Precision::Full, "tests")));
    /// assert_eq!("1.2MT", 1234567u32.human_count_as((System::SI2, "T", false, 1)));
    /// ```
    fn human_count_as<T: IntoHumanCountRepr>(self, repr: impl Into<Option<T>>) -> HumanCountData;
}

macro_rules! impl_trait {
    ($($t:ty),+) => {$(
        impl HumanCount for $t {
            #[inline]
            fn human_count_as<T: IntoHumanCountRepr>(self, repr: impl Into<Option<T>>) -> HumanCountData {
                HumanCountData {
                    val: self as f64,
                    repr:  repr.into().map(IntoHumanCountRepr::into_repr),
                }
            }
        }
    )+}
}
impl_trait!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);
