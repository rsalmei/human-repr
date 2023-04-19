#![cfg_attr(
    not(any(feature = "1024", feature = "iec", feature = "space")), 
    doc = include_str!("../README.md")
)]

mod human_count;
mod human_duration;
mod human_throughput;
mod utils;

use std::borrow::Cow;

/// Human Count data, ready to generate Debug and Display representations.
#[derive(PartialEq, PartialOrd)] // Debug and Display impls in the specific module.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HumanCountData<'a> {
    val: f64,
    unit: Cow<'a, str>,
}

/// Human Duration data, ready to generate Debug and Display representations.
#[derive(PartialEq, PartialOrd)] // Debug and Display impls in the specific module.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HumanDurationData {
    val: f64,
}

/// Human Throughput data, ready to generate Debug and Display representations.
#[derive(PartialEq, PartialOrd)] // Debug and Display impls in the specific module.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HumanThroughputData<'a> {
    val: f64,
    unit: Cow<'a, str>,
}

const BYTES: &str = "B";

/// Human Count trait, supporting all Rust primitive number types.
pub trait HumanCount: sealed::Sealed + Sized {
    /// Generate beautiful human-readable counts supporting automatic prefixes and custom units.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "space")),
        doc = r#"

```
use human_repr::HumanCount;
assert_eq!("4.2Mcoins", 4221432u32.human_count("coins"));
```
"#
    )]
    fn human_count<'a>(self, unit: impl Into<Cow<'a, str>>) -> HumanCountData<'a>;

    /// Generate beautiful human-readable counts supporting automatic prefixes.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "space")),
        doc = r#"

```
use human_repr::HumanCount;
assert_eq!("4.2M", 4221432u32.human_count_bare());
```
"#
    )]
    fn human_count_bare(self) -> HumanCountData<'static> {
        self.human_count("")
    }

    /// Generate beautiful human-readable counts supporting automatic prefixes and Bytes `B` as the unit.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "space")),
        doc = r#"

```
use human_repr::HumanCount;
assert_eq!("4.2MB", 4221432u32.human_count_bytes());
```
"#
    )]
    fn human_count_bytes(self) -> HumanCountData<'static> {
        self.human_count(BYTES)
    }
}

/// Human Duration trait, supporting all Rust primitive number types and Duration.
pub trait HumanDuration: sealed::Sealed + Sized {
    /// Generate beautiful human-readable durations supporting automatic prefixes.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "space")),
        doc = r#"

Use either with primitives:
```
use human_repr::HumanDuration;
assert_eq!("160ms", 0.1599999.human_duration());
```

Or with [`Duration`](`std::time::Duration`)s:
```
use human_repr::HumanDuration;
use std::time::Duration;

let d = Duration::from_secs_f64(0.1599999);
assert_eq!("160ms", d.human_duration());
```
"#
    )]
    fn human_duration(self) -> HumanDurationData;
}

/// Human Throughput trait, supporting all Rust primitive number types.
pub trait HumanThroughput: sealed::Sealed + Sized {
    /// Generate beautiful human-readable throughputs supporting automatic prefixes and custom units.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "space")),
        doc = r#"

```
use human_repr::HumanThroughput;
assert_eq!("1.2k°C/s", 1234.5.human_throughput("°C"));
```
"#
    )]
    fn human_throughput<'a>(self, unit: impl Into<Cow<'a, str>>) -> HumanThroughputData<'a>;

    /// Generate beautiful human-readable throughputs supporting automatic prefixes.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "space")),
        doc = r#"

```
use human_repr::HumanThroughput;
assert_eq!("1.2k/s", 1234.5.human_throughput_bare());
```
"#
    )]
    fn human_throughput_bare(self) -> HumanThroughputData<'static> {
        self.human_throughput("")
    }

    /// Generate beautiful human-readable throughputs supporting automatic prefixes and Bytes `B` as the unit.
    #[cfg_attr(
        not(any(feature = "1024", feature = "iec", feature = "space")),
        doc = r#"

```
use human_repr::HumanThroughput;
assert_eq!("1.2kB/s", 1234.5.human_throughput_bytes());
```
"#
    )]
    fn human_throughput_bytes(self) -> HumanThroughputData<'static> {
        self.human_throughput(BYTES)
    }
}

macro_rules! impl_trait {
    {$($t:ty),+} => {$(
        impl HumanCount for $t {
            fn human_count<'a>(self, unit: impl Into<Cow<'a, str>>) -> HumanCountData<'a> {
                HumanCountData{val: self as f64, unit: unit.into()}
            }
        }
        impl HumanDuration for $t {
            fn human_duration(self) -> HumanDurationData {
                HumanDurationData{val: self as f64}
            }
        }
        impl HumanThroughput for $t {
            fn human_throughput<'a>(self, unit: impl Into<Cow<'a, str>>) -> HumanThroughputData<'a> {
                HumanThroughputData{val: self as f64, unit: unit.into()}
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
