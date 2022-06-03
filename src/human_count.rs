use super::{rounded, HumanCount, SPACE};
use std::fmt;

// Not enabling any optional features gets: SI symbols, divisor is 1000, and with space.
const SPEC: &[&str] = {
    match (cfg!(feature = "iec"), cfg!(feature = "1024")) {
        (true, _) => &["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"], // IEC (1024)
        (false, false) => &["", "k", "M", "G", "T", "P", "E", "Z", "Y"],    // SI (1000).
        (false, true) => &["", "K", "M", "G", "T", "P", "E", "Z", "Y"], // IEC (without "i" prefixes).
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
                r if r.fract() == 0. => return write!(f, "{r:.0}{SPACE}{scale}{unit}"),
                r if (r * 10.).fract() == 0. => return write!(f, "{r:.1}{SPACE}{scale}{unit}"),
                r => return write!(f, "{r:.dec$}{SPACE}{scale}{unit}"),
            }
        }

        write!(f, "{val:.2}{SPACE}+{unit}")
    }
}

}

#[cfg(test)]
mod tests {
    use crate::HumanRepr;

    #[test]
    fn operation() {
        assert_eq!("123 kB", 123000_u64.human_count_bytes());
        assert_eq!("123.5 kB", 123456_u64.human_count_bytes());
        assert_eq!("23 B", 23u8.human_count_bytes());
        assert_eq!("23 B", 23i8.human_count_bytes());
        assert_eq!("-23 B", (-23i8).human_count_bytes());
        assert_eq!("1 kB", 1025u16.human_count_bytes());
        assert_eq!("-1 kB", (-1025i16).human_count_bytes());
        assert_eq!("43.2 MB", 43214321u32.human_count_bytes());
        assert_eq!("23.4 GB", 23403454432_u64.human_count_bytes());
        assert_eq!("23.43 GB", 23433454432_u64.human_count_bytes());
        assert_eq!("18.45 EB", u64::MAX.human_count_bytes());
        assert_eq!("9.22 EB", i64::MAX.human_count_bytes());
        assert_eq!("-9.22 EB", i64::MIN.human_count_bytes());
        assert_eq!("340282366920.94 +B", u128::MAX.human_count_bytes());
    }

    #[test]
    fn flexibility() {
        assert_eq!("123 MCrabs", 123e6.human_count("Crabs"));
        assert_eq!("123 MCrabs", 123e6.human_count("Crabs".to_owned()));
        assert_eq!("123 k🦀", 123e3.human_count("🦀"));
    }

    #[test]
    fn ownership() {
        let mut a = 42000;
        assert_eq!("42 kB", a.human_count_bytes());
        assert_eq!("42 kB", (&a).human_count_bytes());
        assert_eq!("42 kB", (&mut a).human_count_bytes());
    }
}
