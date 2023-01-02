#![doc = include_str!("../README.md")]

mod human_count;
mod human_duration;
mod human_throughput;
mod utils;

use std::time::Duration;

/// Human count repr generator.
pub struct HumanCountData<T>(f64, T, Option<i8>);
/// Human duration repr generator.
pub struct HumanDurationData(f64);
/// Human throughput repr generator.
pub struct HumanThroughputData<T>(f64, T, Option<i8>);

const BYTES: &str = "B";

/// Human representation count trait, already implemented for all Rust primitive number types.
pub trait HumanCount: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable count supporting automatic scales, a custom unit, and optional
    /// fixed precision.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.23Mcoins", 4228432u32.human_count_with("coins", 2));
    /// ```
    fn human_count_with<T>(self, unit: T, precision: impl Into<Option<i8>>) -> HumanCountData<T>;

    /// Generate a beautiful human-readable count supporting automatic scales and a custom unit.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2Mcoins", 4228432u32.human_count("coins"));
    /// ```
    fn human_count<T>(self, unit: T) -> HumanCountData<T> {
        self.human_count_with(unit, None)
    }

    /// Generate a beautiful human-readable count supporting automatic scales and no unit.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2M", 4228432u32.human_count_bare());
    /// ```
    fn human_count_bare(self) -> HumanCountData<&'static str> {
        self.human_count("")
    }

    /// Generate a beautiful human-readable count supporting automatic scales and bytes `B` as the unit.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2MB", 4228432u32.human_count_bytes());
    /// ```
    fn human_count_bytes(self) -> HumanCountData<&'static str> {
        self.human_count(BYTES)
    }
}

/// Human representation duration trait, already implemented for all Rust primitive number types.
pub trait HumanDuration: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable duration supporting SI prefixes nanos (`ns`), micros (`µs`),
    /// millis (`ms`), and seconds (`s`), in addition to minutes (`M:SS`) and even hours (`H:MM:SS`).
    ///
    /// Use either with primitives:
    /// ```
    /// use human_repr::HumanDuration;
    /// assert_eq!("160ms", 0.1599999.human_duration());
    /// ```
    ///
    /// Or with [`Duration`](`std::time::Duration`)s:
    /// ```
    /// use human_repr::HumanDuration;
    /// use std::time::Duration;
    ///
    /// let d = Duration::from_secs_f64(0.1599999);
    /// assert_eq!("160ms", d.human_duration());
    /// ```
    fn human_duration(self) -> HumanDurationData;
}

/// Human representation throughput trait, already implemented for all Rust primitive number types.
pub trait HumanThroughput: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable throughput supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), a custom unit, and optional fixed precision.
    ///
    /// ```
    /// use human_repr::HumanThroughput;
    /// assert_eq!("1.2k°C/s", 1234.5.human_throughput("°C"));
    /// ```
    fn human_throughput_with<T>(
        self,
        unit: T,
        precision: impl Into<Option<i8>>,
    ) -> HumanThroughputData<T>;

    /// Generate a beautiful human-readable throughput supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and a custom unit.
    ///
    /// ```
    /// use human_repr::HumanThroughput;
    /// assert_eq!("1.2k°C/s", 1234.5.human_throughput("°C"));
    /// ```
    fn human_throughput<T>(self, unit: T) -> HumanThroughputData<T> {
        self.human_throughput_with(unit, None)
    }

    /// Generate a beautiful human-readable throughput supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and no unit.
    ///
    /// ```
    /// use human_repr::HumanThroughput;
    /// assert_eq!("1.2k/s", 1234.5.human_throughput_bare());
    /// ```
    fn human_throughput_bare(self) -> HumanThroughputData<&'static str> {
        self.human_throughput("")
    }

    /// Generate a beautiful human-readable throughput supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and bytes `B` as the unit.
    ///
    /// ```
    /// use human_repr::HumanThroughput;
    /// assert_eq!("1.2kB/s", 1234.5.human_throughput_bytes());
    /// ```
    fn human_throughput_bytes(self) -> HumanThroughputData<&'static str> {
        self.human_throughput(BYTES)
    }
}

macro_rules! impl_trait {
    {$($t:ty),+} => {$(
        impl HumanCount for $t {
            fn human_count_with<T>(self, unit: T, precision: impl Into<Option<i8>>) -> HumanCountData<T> {
                HumanCountData(self as f64, unit, precision.into())
            }
        }
        impl HumanDuration for $t {
            fn human_duration(self) -> HumanDurationData {
                HumanDurationData(self as f64)
            }
        }
        impl HumanThroughput for $t {
            fn human_throughput_with<T>(self, unit: T, precision: impl Into<Option<i8>>) -> HumanThroughputData<T> {
                HumanThroughputData(self as f64, unit, precision.into())
            }
        }
    )+}
}
impl_trait!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);

impl HumanDuration for Duration {
    fn human_duration(self) -> HumanDurationData {
        self.into()
    }
}

mod sealed {
    use std::time::Duration;

    pub trait Sealed {}
    macro_rules! impl_sealed {
        {$($t:ty),+} => {
            $(impl Sealed for $t {})+
        }
    }
    impl_sealed!(
        u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize, Duration
    );
}
