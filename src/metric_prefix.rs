use crate::repr_base::{Precision, Separator};
use crate::utils;
use std::fmt;
use std::fmt::Formatter;

/// The system used to represent metric prefixes.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum System {
    /// SI system (1000 divisor).
    SI,
    /// SI system with binary prefix (1024 divisor).
    SI2,
    /// IEC system (1024 divisor).
    IEC,
}

impl System {
    fn system_divisor(sys: Option<System>) -> (System, f64) {
        const SYSTEM_DEFAULT: System = match (cfg!(feature = "iec"), cfg!(feature = "1024")) {
            (false, false) => System::SI,
            (false, true) => System::SI2,
            (true, _) => System::IEC,
        };
        match sys.unwrap_or(SYSTEM_DEFAULT) {
            System::SI => (System::SI, 1000.),
            System::SI2 => (System::SI2, 1024.),
            System::IEC => (System::IEC, 1024.),
        }
    }
}

/// Human metric prefix representation for large values, i.e., abs(val) >= 1.
pub fn human_repr(
    mut val: f64,
    unit: &str,
    sys: Option<System>,
    prec: Option<Precision>,
    sep: Separator, // separator varies per entity, so it should already be resolved here.
    f: &mut Formatter<'_>,
) -> fmt::Result {
    const M: usize = 11;
    const SPEC_SI: [&str; M] = ["", "k", "M", "G", "T", "P", "E", "Z", "Y", "R", "Q"];
    const SPEC_SI2: [&str; M] = const {
        let mut spec = SPEC_SI;
        spec[1] = "K"; // only k is different from SI (1000).
        spec
    };
    const SPEC_IEC: [&str; M] = [
        "", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi", "Ri", "Qi",
    ];

    let (sys, div) = System::system_divisor(sys);
    let spec = match sys {
        System::SI => &SPEC_SI,
        System::SI2 => &SPEC_SI2,
        System::IEC => &SPEC_IEC,
    };

    if val < 0. {
        write!(f, "-")?;
        val = -val;
    }

    let mut it = spec.iter().peekable();
    while let Some(prefix) = it.next() {
        match utils::rounded(val, 2) {
            r if r >= div && it.peek().is_some() => {
                val /= div;
            }
            r => {
                match prec {
                    Some(Precision::Full) => write!(f, "{val}")?, // full precision for serde.
                    Some(Precision::Fixed(dec)) => write!(f, "{val:.*}", dec as usize)?,
                    None => write!(f, "{r:.*}", utils::min_decimals(r))?, // rounded with up to dec decimals.
                }
                return match sep == Separator::Yes && prefix.is_empty() && unit.is_empty() {
                    true => Ok(()), // avoid "123 ".
                    false => write!(f, "{sep}{prefix}{unit}"),
                };
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! base {
        ($val:expr, $unit:expr, $sys:expr, $prec:expr, $sep:expr) => {{
            struct H;
            impl fmt::Display for H {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    human_repr($val as f64, $unit, $sys, $prec, $sep, f)
                }
            }
            format!("{H}")
        }};
    }

    #[test]
    fn operation() {
        macro_rules! case {
            ($val:expr) => {
                base!($val, "", Some(System::SI), None, Separator::No)
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
                base!($val, "", Some(System::SI), $prec.into(), Separator::No)
            };
        }

        use Precision as P;
        assert_eq!("123k", case!(123000_u64, None));
        assert_eq!("123.46k", case!(123458_u64, None));
        assert_eq!("23.51", case!(23.5123, None));
        assert_eq!("-23", case!(-23i8, None));
        assert_eq!("1.02k", case!(1025u16, None));
        assert_eq!("1.2k", case!(1200u16, None));
        assert_eq!("-0.23", case!(-0.23403454432, None));
        assert_eq!("23.4G", case!(23.4e9, None));
        assert_eq!("1.12R", case!(1.123456e27, None));

        assert_eq!("123.0k", case!(123000_u64, P::Fixed(1)));
        assert_eq!("123.5k", case!(123456_u64, P::Fixed(1)));
        assert_eq!("24", case!(23.5123, P::Fixed(0)));
        assert_eq!("-23.0", case!(-23i8, P::Fixed(1)));
        assert_eq!("1.025k", case!(1025u16, P::Fixed(3)));
        assert_eq!("1.20k", case!(1200u16, P::Fixed(2)));
        assert_eq!("0.2340", case!(0.23403454432, P::Fixed(4)));
        assert_eq!("23.40G", case!(23.4e9, P::Fixed(2)));
        assert_eq!("1.12345600R", case!(1.123456e27, P::Fixed(8)));

        assert_eq!("123k", case!(123000_u64, P::Full));
        assert_eq!("123.456k", case!(123456_u64, P::Full));
        assert_eq!("987.56789012", case!(987.56789012, P::Full));
        assert_eq!("0.23403454432", case!(0.23403454432, P::Full));
        assert_eq!("1.23456723403454M", case!(1234567.23403454, P::Full));
        assert_eq!("1.123456R", case!(1.123456e27, P::Full));
    }

    #[test]
    fn units() {
        macro_rules! case {
            ($val:expr, $unit:expr) => {
                base!($val, $unit, Some(System::SI), None, Separator::No)
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
    fn separator() {
        macro_rules! case {
            ($val:expr, $unit:expr) => {
                base!($val, $unit, Some(System::SI), None, Separator::Yes)
            };
        }
        assert_eq!("123", case!(123, ""));
        assert_eq!("123 Crabs", case!(123, "Crabs"));
        assert_eq!("123 🦀", case!(123, "🦀"));
        assert_eq!("123 °C", case!(123, "°C"));

        assert_eq!("123.5", case!(123.5, ""));
        assert_eq!("123.5 Crabs", case!(123.5, "Crabs"));
        assert_eq!("123.5 🦀", case!(123.5, "🦀"));
        assert_eq!("123.5 °C", case!(123.5, "°C"));

        assert_eq!("123 k", case!(123e3, ""));
        assert_eq!("123 kCrabs", case!(123e3, "Crabs"));
        assert_eq!("123 k🦀", case!(123e3, "🦀"));
        assert_eq!("123 k°C", case!(123e3, "°C"));
    }

    #[test]
    fn systems() {
        macro_rules! case {
            ($val:expr, $sys:expr) => {
                base!($val, "", Some($sys), None, Separator::No)
            };
        }

        use System as S;
        assert_eq!("1Ki", case!(1024, S::IEC));
        assert_eq!("1Mi", case!(1048576, S::IEC));
        assert_eq!("1Gi", case!(1073741824, S::IEC));
        assert_eq!("1Ti", case!(1099511627776u64, S::IEC));
        assert_eq!("1Pi", case!(1125899906842624u64, S::IEC));
        assert_eq!("1Ei", case!(1152921504606846976u64, S::IEC));

        assert_eq!("1.02k", case!(1024, S::SI));
        assert_eq!("1K", case!(1024, S::SI2));
        assert_eq!("1.05M", case!(1048576, S::SI));
        assert_eq!("1M", case!(1048576, S::SI2));
        assert_eq!("1.07G", case!(1073741824, S::SI));
        assert_eq!("1G", case!(1073741824, S::SI2));
        assert_eq!("1.1T", case!(1099511627776u64, S::SI));
        assert_eq!("1T", case!(1099511627776u64, S::SI2));
        assert_eq!("1.13P", case!(1125899906842624u64, S::SI));
        assert_eq!("1P", case!(1125899906842624u64, S::SI2));
        assert_eq!("1.15E", case!(1152921504606846976u64, S::SI));
        assert_eq!("1E", case!(1152921504606846976u64, S::SI2));
    }
}
