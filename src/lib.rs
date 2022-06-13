#![doc = include_str!("../README.md")]

mod human_count;
mod human_duration;
mod human_throughput;

#[derive(Debug)]
pub struct HumanCount<T>(f64, T);
#[derive(Debug)]
pub struct HumanDuration(f64);
#[derive(Debug)]
pub struct HumanThroughput<T>(f64, T);

const BYTES: &str = "B";

/// Human representation trait, already implemented for all Rust primitive number types.
pub trait HumanRepr: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable count, supporting SI prefixes and others.
    /// <br>If an even larger scale would be needed (possible with a [u128]), a "+" is used.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("43.2 Mcoins", 43214321u32.human_count("coins"));
    /// ```
    fn human_count<T>(self, unit: T) -> HumanCount<T>;

    /// Generate a beautiful human-readable count, using `"B"` as the unit.
    ///
    /// See [Self::human_count()].
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("43.2 MB", 43214321u32.human_count_bytes());
    /// ```
    fn human_count_bytes(self) -> HumanCount<&'static str> {
        self.human_count(BYTES)
    }

    /// Generate a beautiful human-readable duration, supporting nanos (`ns`), millis (`ms`),
    /// micros (`µs`), seconds (`s`), minutes (`M:SS`), and even hours (`H:MM:SS`).
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("160 ms", 0.1599999.human_duration());
    /// ```
    fn human_duration(self) -> HumanDuration;

    /// Generate a beautiful human-readable throughput, supporting per-day (`/d`), per-hour (`/h`),
    /// per-month (`/m`), and per-sec (`/s`).
    /// <br>When in `/s`, SI prefixes can appear, as in [Self::human_count()].
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2 Mcoins/s", 1234567.8.human_throughput("coins"));
    /// ```
    fn human_throughput<T>(self, unit: T) -> HumanThroughput<T>;

    /// Generate a beautiful human-readable throughput, using `"B"` as the unit.
    ///
    /// See [Self::human_throughput()].
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2 MB/s", 1234567.8.human_throughput_bytes());
    /// ```
    fn human_throughput_bytes(self) -> HumanThroughput<&'static str> {
        self.human_throughput(BYTES)
    }
}

pub trait HumanReprDuration: sealed::Sealed + Sized {
    /// Generate a beautiful human-readable duration, supporting nanos (`ns`), millis (`ms`),
    /// micros (`µs`), seconds (`s`), minutes (`M:SS`), and even hours (`H:MM:SS`).
    ///
    /// ```
    /// use human_repr::HumanReprDuration;
    /// use std::time::Duration;
    ///
    /// let d = Duration::from_secs_f64(0.1599999);
    /// assert_eq!("160 ms", d.human_duration());
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
        1 => (val * 10.).round() / 10.,
        2 => (val * 100.).round() / 100.,
        // 0 => val.round(),
        _ => unreachable!(),
    }
}
