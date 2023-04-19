#![doc = include_str!("../README.md")]

mod human_count;
mod human_duration;
mod human_throughput;
mod utils;

/// Human count data, ready to generate Debug and Display representations.
#[derive(PartialEq, PartialOrd)] // Debug and Display impls in the specific module.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HumanCountData<T> {
    val: f64,
    unit: T,
}

/// Human duration data, ready to generate Debug and Display representations.
#[derive(PartialEq, PartialOrd)] // Debug and Display impls in the specific module.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HumanDurationData {
    val: f64,
}

/// Human throughput data, ready to generate Debug and Display representations.
#[derive(PartialEq, PartialOrd)] // Debug and Display impls in the specific module.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HumanThroughputData<T> {
    val: f64,
    unit: T,
}

const BYTES: &str = "B";

/// Human representation count trait, supporting all Rust primitive number types.
pub trait HumanCount: sealed::Sealed + Sized {
    /// Generate beautiful human-readable counts supporting automatic prefixes and custom units.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2Mcoins", 4221432u32.human_count("coins"));
    /// ```
    fn human_count<T>(self, unit: T) -> HumanCountData<T>;

    /// Generate beautiful human-readable counts supporting automatic prefixes.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2M", 4221432u32.human_count_bare());
    /// ```
    fn human_count_bare(self) -> HumanCountData<&'static str> {
        self.human_count("")
    }

    /// Generate beautiful human-readable counts supporting automatic prefixes and Bytes `B` as the unit.
    ///
    /// ```
    /// use human_repr::HumanCount;
    /// assert_eq!("4.2MB", 4221432u32.human_count_bytes());
    /// ```
    fn human_count_bytes(self) -> HumanCountData<&'static str> {
        self.human_count(BYTES)
    }
}

/// Human representation duration trait, supporting all Rust primitive number types and Duration.
pub trait HumanDuration: sealed::Sealed + Sized {
    /// Generate beautiful human-readable durations supporting automatic prefixes.
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

/// Human representation throughput trait, supporting all Rust primitive number types.
pub trait HumanThroughput: sealed::Sealed + Sized {
    /// Generate beautiful human-readable throughputs supporting automatic prefixes and custom units.
    ///
    /// ```
    /// use human_repr::HumanThroughput;
    /// assert_eq!("1.2k°C/s", 1234.5.human_throughput("°C"));
    /// ```
    fn human_throughput<T>(self, unit: T) -> HumanThroughputData<T>;

    /// Generate beautiful human-readable throughputs supporting automatic prefixes.
    ///
    /// ```
    /// use human_repr::HumanThroughput;
    /// assert_eq!("1.2k/s", 1234.5.human_throughput_bare());
    /// ```
    fn human_throughput_bare(self) -> HumanThroughputData<&'static str> {
        self.human_throughput("")
    }

    /// Generate beautiful human-readable throughputs supporting automatic prefixes and Bytes `B` as the unit.
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
                HumanCountData{val: self as f64, unit}
            }
        }
        impl HumanDuration for $t {
            fn human_duration(self) -> HumanDurationData {
                HumanDurationData{val: self as f64}
            }
        }
        impl HumanThroughput for $t {
            fn human_throughput<T>(self, unit: T) -> HumanThroughputData<T> {
                HumanThroughputData{val: self as f64, unit}
            }
        }
    )+}
}
impl_trait!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);

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
