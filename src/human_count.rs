use super::{rounded, HumanCount, SPACE};
use std::{fmt, ops};

// Not enabling any optional features gets: SI symbols, divisor is 1000, and with space.
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

impl<T: AsRef<str>> fmt::Display for HumanCount<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (mut val, unit) = (self.0, self.1.as_ref());
        for (&scale, &dec) in SPEC.iter().zip(DECIMALS) {
            match rounded(val, dec) {
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

impl<T: AsRef<str>> fmt::Debug for HumanCount<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("HumanCount");
        ds.field("val", &self.0);
        if !self.1.as_ref().is_empty() {
            ds.field("unit", &self.1.as_ref());
        }
        ds.finish()?;
        write!(f, " -> ")?;
        fmt::Display::fmt(self, f)
    }
}

impl<T: AsRef<str>> PartialEq<HumanCount<T>> for &str {
    fn eq(&self, other: &HumanCount<T>) -> bool {
        super::display_compare(self, other)
    }
}

impl<T: AsRef<str>> PartialEq<&str> for HumanCount<T> {
    fn eq(&self, other: &&str) -> bool {
        other == self
    }
}

impl<T> ops::Neg for HumanCount<T> {
    type Output = HumanCount<T>;

    fn neg(self) -> Self::Output {
        HumanCount(-self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::HumanRepr;

    #[test]
    fn operation() {
        assert_eq!("123kB", 123000_u64.human_count_bytes());
        assert_eq!("123.5kB", 123456_u64.human_count_bytes());
        assert_eq!("23B", 23u8.human_count_bytes());
        assert_eq!("23B", 23i8.human_count_bytes());
        assert_eq!("23.5B", 23.5123.human_count_bytes());
        assert_eq!("-23B", -23i8.human_count_bytes());
        assert_eq!("1kB", 1025u16.human_count_bytes());
        assert_eq!("-1kB", -1025i16.human_count_bytes());
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
        assert_eq!("123k🦀", 123e3.human_count("🦀"));
        assert_eq!("12.3k°C", 123e2.human_count("°C"));
        assert_eq!("1.2°C", 123e-2.human_count("°C"));
    }

    #[test]
    fn ownership() {
        let mut a = 42000;
        assert_eq!("42kB", a.human_count_bytes());
        assert_eq!("42kB", (&a).human_count_bytes());
        assert_eq!("42kB", (&mut a).human_count_bytes());
    }
}
