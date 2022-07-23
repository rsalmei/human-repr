#![doc = include_str!("../README.md")]

mod human_count;
mod human_duration;
mod human_throughput;

use std::fmt;

pub struct HumanCount<T>(f64, T);
pub struct HumanDuration(f64);
pub struct HumanThroughput<T>(f64, T);

const BYTES: &str = "B";

/// Human representation trait, already implemented for all Rust primitive number types.
pub trait HumanRepr: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable count, supporting SI/IEC prefixes to indicate multiples,
    /// and your custom unit.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("4.2Mcoins", 4221432u32.human_count("coins"));
    /// ```
    fn human_count<T>(self, unit: T) -> HumanCount<T>;

    /// Generate a beautiful human-readable count, supporting SI/IEC prefixes to indicate multiples,
    /// and an empty unit.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("4.2M", 4221432u32.human_count_bare());
    /// ```
    fn human_count_bare(self) -> HumanCount<&'static str> {
        self.human_count("")
    }

    /// Generate a beautiful human-readable count, supporting SI/IEC prefixes to indicate multiples,
    /// and bytes `"B"` as the unit.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("4.2MB", 4221432u32.human_count_bytes());
    /// ```
    fn human_count_bytes(self) -> HumanCount<&'static str> {
        self.human_count(BYTES)
    }

    /// Generate a beautiful human-readable duration, supporting SI prefixes nanos (`ns`), micros (`µs`),
    /// millis (`ms`), and seconds (`s`), in addition to minutes (`M:SS`) and even hours (`H:MM:SS`).
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("160ms", 0.1599999.human_duration());
    /// ```
    fn human_duration(self) -> HumanDuration;

    /// Generate a beautiful human-readable throughput, supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and your custom unit.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2k°C/s", 1234.5.human_throughput("°C"));
    /// ```
    fn human_throughput<T>(self, unit: T) -> HumanThroughput<T>;

    /// Generate a beautiful human-readable throughput, supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and an empty unit.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2k/s", 1234.5.human_throughput_bare());
    /// ```
    fn human_throughput_bare(self) -> HumanThroughput<&'static str> {
        self.human_throughput("")
    }

    /// Generate a beautiful human-readable throughput, supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`), and bytes `"B"` as the unit.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2kB/s", 1234.5.human_throughput_bytes());
    /// ```
    fn human_throughput_bytes(self) -> HumanThroughput<&'static str> {
        self.human_throughput(BYTES)
    }
}

pub trait HumanReprDuration: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable duration, supporting SI prefixes nanos (`ns`), micros (`µs`),
    /// millis (`ms`), and seconds (`s`), in addition to minutes (`M:SS`) and even hours (`H:MM:SS`).
    ///
    /// ```
    /// use human_repr::HumanReprDuration;
    /// use std::time::Duration;
    ///
    /// let d = Duration::from_secs_f64(0.1599999);
    /// assert_eq!("160ms", d.human_duration());
    /// ```
    fn human_duration(self) -> HumanDuration;
}

macro_rules! impl_trait {
    {$($t:ty),+} => {$(
        impl HumanRepr for $t {
            fn human_count<T>(self, unit: T) -> HumanCount<T> {
                HumanCount(self as f64, unit)
            }
            fn human_duration(self) -> HumanDuration {
                HumanDuration(self as f64)
            }
            fn human_throughput<T>(self, unit: T) -> HumanThroughput<T> {
                HumanThroughput(self as f64, unit)
            }
        }
    )+}
}
impl_trait!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);

mod sealed {
    pub trait Sealed {}
    macro_rules! impl_sealed {
        {$($t:ty),+} => {
            $(impl Sealed for $t {})+
        }
    }
    impl_sealed!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);
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

pub fn display_compare(str: &str, display: &impl fmt::Display) -> bool {
    let mut it = str.bytes();
    use fmt::Write;
    write!(DisplayCompare(it.by_ref()), "{display}").map_or(false, |_| it.len() == 0)
}
