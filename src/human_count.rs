use super::HumanCountData;
use crate::utils::{self, SPACE};
use std::fmt::{self, Debug, Display};

// with default features we get: SI symbols, 1000 divisor, and no spaces.
const SPEC: &[&str] = {
    match (cfg!(feature = "iec"), cfg!(feature = "1024")) {
        (false, false) => &["", "k", "M", "G", "T", "P", "E", "Z", "Y"], // SI (1000).
        (false, true) => &["", "K", "M", "G", "T", "P", "E", "Z", "Y"],  // SI (1024).
        (true, _) => &["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"], // IEC (1024)
    }
};
const DECIMALS: &[usize] = &[1, 1, 1, 2, 2, 2, 2, 2, 2];
const DIVISOR: f64 = {
    match cfg!(feature = "1024") {
        true => 1024.,
        false => 1000.,
    }
};

impl Display for HumanCountData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HumanCountData { mut val, unit } = self;
        for (&scale, &dec) in SPEC.iter().zip(DECIMALS) {
            match utils::rounded(val, dec) {
                r if r.abs() >= DIVISOR => val /= DIVISOR,
                r if r.fract() == 0. => return write!(f, "{:.0}{}{}{}", r, SPACE, scale, unit),
                r if (r * 10.).fract() == 0. => {
                    return write!(f, "{:.1}{}{}{}", r, SPACE, scale, unit)
                }
                r => return write!(f, "{:.2}{}{}{}", r, SPACE, scale, unit),
            }
        }

        write!(f, "{:.2}{}+{}", val, SPACE, unit)
    }
}

impl Debug for HumanCountData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("HumanCount");
        ds.field("val", &self.val);
        ds.field("unit", &self.unit);
        ds.finish()?;
        write!(f, " -> ")?;
        fmt::Display::fmt(self, f)
    }
}

impl PartialEq<HumanCountData<'_>> for &str {
    fn eq(&self, other: &HumanCountData<'_>) -> bool {
        utils::display_compare(self, other)
    }
}

impl PartialEq<&str> for HumanCountData<'_> {
    fn eq(&self, other: &&str) -> bool {
        other == self
    }
}

#[cfg(all(test, not(any(feature = "1024", feature = "iec", feature = "space"))))]
mod tests {
    use crate::HumanCount;

    #[test]
    fn operation() {
        assert_eq!("123kB", 123000_u64.human_count_bytes());
        assert_eq!("123.5kB", 123456_u64.human_count_bytes());
        assert_eq!("23B", 23u8.human_count_bytes());
        assert_eq!("23B", 23i8.human_count_bytes());
        assert_eq!("23.5B", 23.5123.human_count_bytes());
        assert_eq!("-23B", (-23i8).human_count_bytes());
        assert_eq!("1kB", 1025u16.human_count_bytes());
        assert_eq!("-1kB", (-1025i16).human_count_bytes());
        assert_eq!("43.2MB", 43214321u32.human_count_bytes());
        assert_eq!("23.4GB", 23403454432_u64.human_count_bytes());
        assert_eq!("23.43GB", 23433454432_u64.human_count_bytes());
        assert_eq!("18.45EB", u64::MAX.human_count_bytes());
        assert_eq!("9.22EB", i64::MAX.human_count_bytes());
        assert_eq!("-9.22EB", i64::MIN.human_count_bytes());
        assert_eq!("340282366920.94+B", u128::MAX.human_count_bytes());
    }

    #[test]
    fn flexibility() {
        assert_eq!("123MCrabs", 123e6.human_count("Crabs"));
        assert_eq!("123MCrabs", 123e6.human_count("Crabs".to_owned()));
        assert_eq!("123MCrabs", 123e6.human_count(&"Crabs".to_owned()));
        assert_eq!("123k🦀", 123e3.human_count("🦀"));
        assert_eq!("12.3k°C", 123e2.human_count("°C"));
        assert_eq!("1.2°C", 123e-2.human_count("°C"));
    }

    #[test]
    #[allow(clippy::needless_borrow)]
    fn ownership() {
        let mut a = 42000;
        assert_eq!("42kB", a.human_count_bytes());
        assert_eq!("42kB", (&a).human_count_bytes());
        assert_eq!("42kB", (&mut a).human_count_bytes());
    }

    #[test]
    fn symmetric() {
        assert_eq!(123000_u64.human_count_bytes(), "123kB");
    }
}

#[test]
#[cfg(feature = "serde")]
fn serialize() -> Result<(), serde_json::Error> {
    use crate::HumanCount;
    let h = 123456.human_count("X");
    let ser = serde_json::to_string(&h)?;
    assert_eq!(r#"{"val":123456.0,"unit":"X"}"#, &ser);
    let h2 = serde_json::from_str::<HumanCountData>(&ser)?;
    assert_eq!(h, h2);
    Ok(())
}
