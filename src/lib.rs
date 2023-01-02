#![doc = include_str!("../README.md")]

mod human_count;
mod human_duration;
mod human_throughput;

use std::fmt;
use std::time::Duration;

/// Human count repr generator.
pub struct HumanCountData<T>(f64, T);
/// Human duration repr generator.
pub struct HumanDurationData(f64);
/// Human throughput repr generator.
pub struct HumanThroughputData<T>(f64, T);

const BYTES: &str = "B";

/// Human representation count trait, already implemented for all Rust primitive number types.
pub trait HumanCount: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable count, supporting SI/IEC prefixes to indicate multiples,
    /// and your custom unit.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2Mcoins", 4221432u32.human_count("coins"));
    /// ```
    fn human_count<T>(self, unit: T) -> HumanCountData<T>;

    /// Generate a beautiful human-readable count, supporting SI/IEC prefixes to indicate multiples,
    /// and an empty unit.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2M", 4221432u32.human_count_bare());
    /// ```
    fn human_count_bare(self) -> HumanCountData<&'static str> {
        self.human_count("")
    }

    /// Generate a beautiful human-readable count, supporting SI/IEC prefixes to indicate multiples,
    /// and bytes `"B"` as the unit.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2MB", 4221432u32.human_count_bytes());
    /// ```
    fn human_count_bytes(self) -> HumanCountData<&'static str> {
        self.human_count(BYTES)
    }
}

/// Human representation duration trait, already implemented for all Rust primitive number types.
pub trait HumanDuration: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable duration, supporting SI prefixes nanos (`ns`), micros (`µs`),
    /// millis (`ms`), and seconds (`s`), in addition to minutes (`M:SS`) and even hours (`H:MM:SS`).
    ///
    /// Use either on primitives:
    /// ```
    /// use human_repr::HumanDuration;
    /// assert_eq!("160ms", 0.1599999.human_duration());
    /// ```
    ///
    /// Or on [`Duration`](`std::time::Duration`)s:
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
    /// Generate a beautiful human-readable throughput, supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and your custom unit.
    ///
    /// ```
    /// use human_repr::HumanThroughput;
    /// assert_eq!("1.2k°C/s", 1234.5.human_throughput("°C"));
    /// ```
    fn human_throughput<T>(self, unit: T) -> HumanThroughputData<T>;

    /// Generate a beautiful human-readable throughput, supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and an empty unit.
    ///
    /// ```
    /// use human_repr::HumanThroughput;
    /// assert_eq!("1.2k/s", 1234.5.human_throughput_bare());
    /// ```
    fn human_throughput_bare(self) -> HumanThroughputData<&'static str> {
        self.human_throughput("")
    }

    /// Generate a beautiful human-readable throughput, supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and bytes `"B"` as the unit.
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
            fn human_count<T>(self, unit: T) -> HumanCountData<T> {
                HumanCountData(self as f64, unit)
            }
        }
        impl HumanDuration for $t {
            fn human_duration(self) -> HumanDurationData {
                HumanDurationData(self as f64)
            }
        }
        impl HumanThroughput for $t {
            fn human_throughput<T>(self, unit: T) -> HumanThroughputData<T> {
                HumanThroughputData(self as f64, unit)
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

const SPACE: &str = {
    match cfg!(feature = "space") {
        true => " ",
        false => "",
    }
};

#[inline]
fn rounded(val: f64, dec: usize) -> f64 {
    match dec {
        0 => val.round(),
        1 => (val * 10.).round() / 10.,
        2 => (val * 100.).round() / 100.,
        _ => unreachable!(),
    }
}

struct DisplayCompare<'a, I>(&'a mut I);

impl<I: Iterator<Item = u8>> fmt::Write for DisplayCompare<'_, I> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match s.bytes().zip(self.0.by_ref()).all(|(x, y)| x == y) {
            true => Ok(()),
            false => Err(fmt::Error),
        }
    }
}

fn display_compare(str: &str, display: &impl fmt::Display) -> bool {
    let mut it = str.bytes();
    use fmt::Write;
    write!(DisplayCompare(it.by_ref()), "{display}").map_or(false, |_| it.len() == 0)
}
