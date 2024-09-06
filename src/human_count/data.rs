use crate::human_count::repr::HumanCountRepr;
use crate::{metric_prefix, utils, Precision, System};
use std::fmt::{self, Debug, Display, Formatter};

/// The HumanCount data object.
#[derive(Copy, Clone, PartialEq)]
pub struct HumanCountData {
    pub val: f64,
    pub repr: HumanCountRepr,
}

/// The beautiful HumanCount renderer.
impl Display for HumanCountData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let HumanCountData { val, repr } = *self;
        let (unit, sys, prec, split) = repr.resolve();
        metric_prefix::human_repr(val, unit, sys, prec, split, f)
    }
}

/// The "{data} -> {human}" representation.
impl Debug for HumanCountData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("HumanCountData")
            .field("val", &self.val)
            .field("repr", &format_args!("{}", self.repr))
            .finish()?;
        write!(f, " -> ")?;
        Display::fmt(self, f)
    }
}

impl PartialEq<HumanCountData> for &str {
    fn eq(&self, other: &HumanCountData) -> bool {
        utils::compare_display(self, other)
    }
}

impl PartialEq<&str> for HumanCountData {
    fn eq(&self, other: &&str) -> bool {
        other == self
    }
}

#[cfg(feature = "parse")]
mod parsing {
    use super::*;
    use std::str::FromStr;

    impl FromStr for HumanCountData {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.trim();
            // if !s.ends_with(U::UNIT) { // TODO include unit support when adt_const_params lands in stable.
            //     return Err(format!("invalid unit: must end with {:?}", U::UNIT));
            // }
            // s = &s[..s.len() - U::UNIT.len()];
            let mut mult = 1.0;
            if s.ends_with(|x: char| !x.is_ascii_digit() && x != '.')
                && s.starts_with(|x: char| x.is_ascii_digit() || x == '-' || x == '+')
            {
                let (spec, div) = System::SI.spec_divisor(); // TODO include System support when adt_const_params lands in stable.
                let pos = 1 + spec[1..]
                    .iter()
                    .position(|&prefix| s.ends_with(prefix))
                    .ok_or_else(|| format!("invalid SI prefix: {s}"))?;
                s = s[..s.len() - spec[pos].len()].trim(); // spaces are supported.
                mult = div.powi(pos as _);
            }
            let val = s
                .parse::<f64>()
                .map_err(|_| format!("invalid value: {s}"))?;
            Ok(Self {
                val: val * mult,
                repr: Default::default(),
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        macro_rules! case {
            ($val:expr) => {
                Ok(HumanCountData {
                    val: $val,
                    repr: Default::default(),
                })
            };
            (x $err:literal) => {
                Err::<HumanCountData, _>($err.to_owned())
            };
        }

        #[test]
        fn parse() {
            assert_eq!("64".parse(), case!(64.0));
            assert_eq!("  1".parse(), case!(1.0));
            assert_eq!("128  ".parse(), case!(128.0));
            assert_eq!("     4096  ".parse(), case!(4096.0));

            assert_eq!("64k  ".parse(), case!(64e3));
            assert_eq!("64 k  ".parse(), case!(64e3));
            assert_eq!("64    k  ".parse(), case!(64e3));
            assert_eq!("  2000k  ".parse(), case!(2e6));
            assert_eq!("  2000 k  ".parse(), case!(2e6));
            assert_eq!("  2000    k   ".parse(), case!(2e6));

            assert_eq!("64.0".parse(), case!(64.0));
            assert_eq!("64.".parse(), case!(64.0));
            assert_eq!("+64.1".parse(), case!(64.1));
            assert_eq!("-64.".parse(), case!(-64.0));
            assert_eq!("-64.12".parse(), case!(-64.12));
            assert_eq!("64.0k".parse(), case!(64e3));
            assert_eq!("64.M".parse(), case!(64e6));
            assert_eq!("-64G".parse(), case!(-64e9));
            assert_eq!("+64T".parse(), case!(64e12));

            // const MEGA: f64 = 1024.0 * 1024.0;
            // assert_eq!("64M".parse(), case!(SI => 64e6));
            // assert_eq!("64M".parse(), case!(SI2 => 64.0 * MEGA));
            // assert_eq!("64M".parse(), case!(IEC => 64.0 * MEGA));
            //
            // assert_eq!(" 64   M  ".parse(), case!(SI => 64e6));
            // assert_eq!("   64 M  ".parse(), case!(SI2 => 64.0 * MEGA));
            // assert_eq!("  64   M ".parse(), case!(IEC => 64.0 * MEGA));

            assert_eq!("roger".parse(), case!(x "invalid value: roger"));
            assert_eq!("123!".parse(), case!(x "invalid SI prefix: 123!"));
            assert_eq!("  !123".parse(), case!(x "invalid value: !123"));
            assert_eq!("64K ".parse(), case!(x "invalid SI prefix: 64K"));
            assert_eq!("64m".parse(), case!(x "invalid SI prefix: 64m"));
            assert_eq!(" +64m  ".parse(), case!(x "invalid SI prefix: +64m"));
            assert_eq!("x64m".parse(), case!(x "invalid value: x64m"));
            assert_eq!("64X ".parse(), case!(x "invalid SI prefix: 64X"));
            assert_eq!(" 64°C".parse(), case!(x "invalid SI prefix: 64°C"));
        }
    }
}

#[cfg(feature = "serde")]
mod serializing {
    use super::*;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for HumanCountData {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let data = HumanCountData {
                val: self.val,
                repr: HumanCountRepr {
                    unit: "", // TODO hardcode unit and system until they are supported in parsing.
                    sys: Some(System::SI),
                    prec: Some(Precision::Full), // always serialize with full precision.
                    split: self.repr.split,
                },
            };
            serializer.collect_str(&data)
        }
    }

    impl<'de> Deserialize<'de> for HumanCountData {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = <&str>::deserialize(deserializer)?;
            s.parse().map_err(de::Error::custom)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn serde() -> Result<(), serde_json::Error> {
            let h = HumanCountData {
                val: 123456.0,
                repr: HumanCountRepr {
                    unit: "X",
                    ..Default::default()
                },
            };
            let ser = serde_json::to_string(&h)?;
            assert_eq!(r#""123.456k""#, &ser);
            let h2 = serde_json::from_str::<HumanCountData>(&ser)?;
            assert_eq!(h.val, h2.val); // h2 doesn't have a unit.
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn types() {
        assert_eq!("123", 123_u8.human_count());
        assert_eq!("123", 123_i8.human_count());
        assert_eq!("123", 123_u16.human_count());
        assert_eq!("123", 123_i16.human_count());
        assert_eq!("123", 123_u32.human_count());
        assert_eq!("123", 123_i32.human_count());
        assert_eq!("123", 123_u64.human_count());
        assert_eq!("123", 123_i64.human_count());
        assert_eq!("123", 123_u128.human_count());
        assert_eq!("123", 123_i128.human_count());
        assert_eq!("123", 123_usize.human_count());
        assert_eq!("123", 123_isize.human_count());
        assert_eq!("123", 123_f32.human_count());
        assert_eq!("123", 123_f64.human_count());

        assert_eq!("-123", (-123_i8).human_count());
        assert_eq!("-123", (-123_i16).human_count());
        assert_eq!("-123", (-123_i32).human_count());
        assert_eq!("-123", (-123_i64).human_count());
        assert_eq!("-123", (-123_i128).human_count());
        assert_eq!("-123", (-123_isize).human_count());
        assert_eq!("-123", (-123_f32).human_count());
        assert_eq!("-123", (-123_f64).human_count());
    }

    #[test]
    fn units() {
        assert_eq!("123Crabs", 123.human_count_as("Crabs"));
        assert_eq!("123k🦀", 123e3.human_count_as("🦀"));
        assert_eq!("12.3k°C", 123e2.human_count_as("°C"));
    }

    #[test]
    #[allow(clippy::needless_borrow)]
    fn ownership() {
        let mut a = 42000;
        assert_eq!("42k", a.human_count());
        assert_eq!("42k", (&a).human_count());
        assert_eq!("42k", (&mut a).human_count());
    }

    #[test]
    fn symmetric() {
        assert_eq!(123000_u64.human_count(), "123k");
    }

    #[test]
    fn eq() {
        let c1 = 0.23403454432.human_count();
        assert_eq!("0.23", c1);
        let c2 = 0.234034.human_count();
        assert_eq!("0.23", c2); // same repr.
        assert_ne!(c1, c2); // but different.

        let c3 = 0.234034.human_count_bytes(); // same value.
        assert_eq!("0.23B", c3); // different unit.
        assert_ne!(c2, c3); // also different.
    }
}
