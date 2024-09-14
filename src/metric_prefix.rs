use crate::{utils, Precision, Split, System};
use std::fmt;

pub const NUM_PREFIXES: usize = 11;

impl System {
    const SPEC_SI: [&'static str; NUM_PREFIXES] =
        ["", "k", "M", "G", "T", "P", "E", "Z", "Y", "R", "Q"];
    const SPEC_SI2: [&'static str; NUM_PREFIXES] = const {
        let mut spec = System::SPEC_SI;
        spec[1] = "K"; // only k is different from SI (1000).
        spec
    };
    const SPEC_IEC: [&'static str; NUM_PREFIXES] = [
        "", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi", "Ri", "Qi",
    ];

    pub(crate) fn spec_divisor(self) -> (&'static [&'static str; NUM_PREFIXES], f64) {
        match self {
            System::SI => (&Self::SPEC_SI, 1000.),
            System::SI2 => (&Self::SPEC_SI2, 1024.),
            System::IEC => (&Self::SPEC_IEC, 1024.),
        }
    }
}

/// Human metric prefix representation for large values, i.e., abs(val) >= 1.
pub fn human_repr(
    mut val: f64,
    unit: &str,
    sys: System,     // system varies per entity, so it should already be resolved here.
    prec: Precision, // precision varies per entity, so it should already be resolved here.
    split: Split,    // split varies per entity, so it should already be resolved here.
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    if val < 0. {
        write!(f, "-")?;
        val = -val;
    }

    let (spec, div) = sys.spec_divisor();
    let mut it = spec.iter().peekable();
    while let Some(pref) = it.next() {
        match utils::rounded(val, 2) {
            r if r >= div && it.peek().is_some() => {
                val /= div;
            }
            r => {
                use {Precision::*, Split::*};
                match prec {
                    Auto => write!(f, "{r:.*}", utils::min_decimals(r))?, // rounded up to dec decimals.
                    Fixed(dec) => write!(f, "{val:.*}", dec as usize)?,
                    Full => write!(f, "{val}")?, // full precision for serde.
                }
                return match split {
                    Join => write!(f, "{pref}{unit}"),
                    Prefix if pref.is_empty() && unit.is_empty() => Ok(()), // avoid "123 ".
                    Unit if unit.is_empty() => write!(f, "{pref}"),         // avoid "123k ".
                    Prefix => write!(f, " {pref}{unit}"),
                    Unit => write!(f, "{pref} {unit}"),
                };
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

    macro_rules! base {
        ($val:expr, $unit:expr, $sys:expr, $prec:expr, $sp:expr) => {{
            struct H;
            impl Display for H {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    human_repr($val as f64, $unit, $sys, $prec, $sp, f)
                }
            }
            format!("{H}")
        }};
    }

    #[test]
    fn operation() {
        macro_rules! case {
            ($val:expr) => {
                base!($val, "", System::SI, Precision::Auto, Split::Join)
            };
        }
        assert_eq!("123k", case!(123000_u64));
        assert_eq!("123.46k", case!(123456_u64));
        assert_eq!("999.96", case!(999.96));
        assert_eq!("23", case!(23u8));
        assert_eq!("23", case!(23i8));
        assert_eq!("-23", case!(-23i8));
        assert_eq!("23.5", case!(23.5));
        assert_eq!("1.02k", case!(1025u16));
        assert_eq!("-1.02k", case!(-1025i16));
        assert_eq!("43.21M", case!(43214321u32));
        assert_eq!("23.4G", case!(23403454432_u64));
        assert_eq!("0.23", case!(0.23403454432));
        assert_eq!("23.43G", case!(23433454432_u64));
        assert_eq!("18.45E", case!(u64::MAX));
        assert_eq!("9.22E", case!(i64::MAX));
        assert_eq!("-9.22E", case!(i64::MIN));
        assert_eq!("1R", case!(999.999e24));
        assert_eq!("1.12R", case!(1.123456e27));
        assert_eq!("1.12Q", case!(1.123456e30));
        assert_eq!("1123.46Q", case!(1.123456e33));
    }

    #[test]
    fn precision() {
        macro_rules! case {
            ($val:expr, $prec:expr) => {
                base!($val, "", System::SI, $prec, Split::Join)
            };
        }

        use Precision::*;
        assert_eq!("123k", case!(123000_u64, Auto));
        assert_eq!("123.46k", case!(123458_u64, Auto));
        assert_eq!("23.51", case!(23.5123, Auto));
        assert_eq!("-23", case!(-23i8, Auto));
        assert_eq!("1.02k", case!(1025u16, Auto));
        assert_eq!("1.2k", case!(1200u16, Auto));
        assert_eq!("-0.23", case!(-0.23403454432, Auto));
        assert_eq!("23.4G", case!(23.4e9, Auto));
        assert_eq!("1.12R", case!(1.123456e27, Auto));

        assert_eq!("123.0k", case!(123000_u64, Fixed(1)));
        assert_eq!("123.5k", case!(123456_u64, Fixed(1)));
        assert_eq!("24", case!(23.5123, Fixed(0)));
        assert_eq!("-23.0", case!(-23i8, Fixed(1)));
        assert_eq!("1.025k", case!(1025u16, Fixed(3)));
        assert_eq!("1.20k", case!(1200u16, Fixed(2)));
        assert_eq!("0.2340", case!(0.23403454432, Fixed(4)));
        assert_eq!("23.40G", case!(23.4e9, Fixed(2)));
        assert_eq!("1.12345600R", case!(1.123456e27, Fixed(8)));

        assert_eq!("123k", case!(123000_u64, Full));
        assert_eq!("123.456k", case!(123456_u64, Full));
        assert_eq!("987.56789012", case!(987.56789012, Full));
        assert_eq!("0.23403454432", case!(0.23403454432, Full));
        assert_eq!("1.23456723403454M", case!(1234567.23403454, Full));
        assert_eq!("1.123456R", case!(1.123456e27, Full));
    }

    #[test]
    fn units() {
        macro_rules! case {
            ($val:expr, $unit:expr) => {
                base!($val, $unit, System::SI, Precision::Auto, Split::Join)
            };
        }
        assert_eq!("123", case!(123, ""));
        assert_eq!("123Crabs", case!(123, "Crabs"));
        assert_eq!("123🦀", case!(123, "🦀"));
        assert_eq!("123°C", case!(123, "°C"));

        assert_eq!("123.5", case!(123.5, ""));
        assert_eq!("123.5Crabs", case!(123.5, "Crabs"));
        assert_eq!("123.5🦀", case!(123.5, "🦀"));
        assert_eq!("123.5°C", case!(123.5, "°C"));

        assert_eq!("123k", case!(123e3, ""));
        assert_eq!("123kCrabs", case!(123e3, "Crabs"));
        assert_eq!("123k🦀", case!(123e3, "🦀"));
        assert_eq!("123k°C", case!(123e3, "°C"));
    }

    #[test]
    fn split() {
        macro_rules! case {
            ($val:expr, $unit:expr, $split:expr) => {
                base!($val, $unit, System::SI, Precision::Auto, $split)
            };
        }
        assert_eq!("123", case!(123, "", Split::Prefix));
        assert_eq!("123 Crabs", case!(123, "Crabs", Split::Prefix));
        assert_eq!("123 🦀", case!(123, "🦀", Split::Prefix));
        assert_eq!("123 °C", case!(123, "°C", Split::Prefix));

        assert_eq!("123", case!(123, "", Split::Unit));
        assert_eq!("123 Crabs", case!(123, "Crabs", Split::Unit));
        assert_eq!("123 🦀", case!(123, "🦀", Split::Unit));
        assert_eq!("123 °C", case!(123, "°C", Split::Unit));

        assert_eq!("123 k", case!(123e3, "", Split::Prefix));
        assert_eq!("123 kCrabs", case!(123e3, "Crabs", Split::Prefix));
        assert_eq!("123 k🦀", case!(123e3, "🦀", Split::Prefix));
        assert_eq!("123 k°C", case!(123e3, "°C", Split::Prefix));

        assert_eq!("123k", case!(123e3, "", Split::Unit));
        assert_eq!("123k Crabs", case!(123e3, "Crabs", Split::Unit));
        assert_eq!("123k 🦀", case!(123e3, "🦀", Split::Unit));
        assert_eq!("123k °C", case!(123e3, "°C", Split::Unit));
    }

    #[test]
    fn systems() {
        macro_rules! case {
            ($val:expr, $sys:expr) => {
                base!($val, "", $sys, Precision::Auto, Split::Join)
            };
        }

        use super::System::*;
        assert_eq!("1Ki", case!(1024, IEC));
        assert_eq!("1Mi", case!(1048576, IEC));
        assert_eq!("1Gi", case!(1073741824, IEC));
        assert_eq!("1Ti", case!(1099511627776u64, IEC));
        assert_eq!("1Pi", case!(1125899906842624u64, IEC));
        assert_eq!("1Ei", case!(1152921504606846976u64, IEC));

        assert_eq!("1.02k", case!(1024, SI));
        assert_eq!("1K", case!(1024, SI2));
        assert_eq!("1.05M", case!(1048576, SI));
        assert_eq!("1M", case!(1048576, SI2));
        assert_eq!("1.07G", case!(1073741824, SI));
        assert_eq!("1G", case!(1073741824, SI2));
        assert_eq!("1.1T", case!(1099511627776u64, SI));
        assert_eq!("1T", case!(1099511627776u64, SI2));
        assert_eq!("1.13P", case!(1125899906842624u64, SI));
        assert_eq!("1P", case!(1125899906842624u64, SI2));
        assert_eq!("1.15E", case!(1152921504606846976u64, SI));
        assert_eq!("1E", case!(1152921504606846976u64, SI2));
    }
}
