use super::{rounded, HumanRepr, HumanThroughput, SPACE};
use std::{fmt, ops};

const SPEC: &[(f64, &str, usize)] = &[
    (24., "/d", 2),
    (60., "/h", 1),
    (60., "/m", 1),
    // "/s" in code.
];

impl<T: AsRef<str>> fmt::Display for HumanThroughput<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (mut val, unit) = (self.0, self.1.as_ref());
        val *= 60. * 60. * 24.;
        for &(size, scale, dec) in SPEC {
            match rounded(val, dec) {
                r if r.abs() >= size => val /= size,
                r if r.fract() == 0. => return write!(f, "{:.0}{}{}{}", r, SPACE, unit, scale),
                r if (r * 10.).fract() == 0. => {
                    return write!(f, "{:.1}{}{}{}", r, SPACE, unit, scale)
                }
                r => return write!(f, "{:.dec$}{}{}{}", r, SPACE, unit, scale, dec = dec),
            }
        }

        write!(f, "{}/s", val.human_count(unit))
    }
}

impl<T: AsRef<str>> fmt::Debug for HumanThroughput<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("HumanThroughput");
        ds.field("val", &self.0);
        if !self.1.as_ref().is_empty() {
            ds.field("unit", &self.1.as_ref());
        }
        ds.finish()?;
        write!(f, " -> ")?;
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl<T: AsRef<str>> PartialEq<HumanThroughput<T>> for &str {
    fn eq(&self, other: &HumanThroughput<T>) -> bool {
        *self == &other.to_string()
    }
}

impl<T: AsRef<str>> PartialEq<&str> for HumanThroughput<T> {
    fn eq(&self, other: &&str) -> bool {
        &self.to_string() == other
    }
}

impl<T> ops::Neg for HumanThroughput<T> {
    type Output = HumanThroughput<T>;

    fn neg(self) -> Self::Output {
        HumanThroughput(-self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::HumanRepr;

    #[test]
    fn operation() {
        assert_eq!("1B/s", 1.human_throughput_bytes());
        assert_eq!("-1B/s", -1.human_throughput_bytes());
        assert_eq!("1.2MB/s", (1234567. / 1.).human_throughput_bytes());
        assert_eq!("10B/s", (10. / 1.).human_throughput_bytes());
        assert_eq!("30B/m", (1. / 2.).human_throughput_bytes());
        assert_eq!("-30B/m", (-1. / 2.).human_throughput_bytes());
        assert_eq!("5B/s", (10. / 2.).human_throughput_bytes());
        assert_eq!("6B/s", (11. / 2.).human_throughput_bytes());
        assert_eq!("6.1B/m", (10. / 99.).human_throughput_bytes());
        assert_eq!("1.8B/m", (3. / 100.).human_throughput_bytes());
        assert_eq!("4.4B/m", (8. / 110.).human_throughput_bytes());
        assert_eq!("6.8B/h", (3. / 1600.).human_throughput_bytes());
        assert_eq!("1.9kB/s", (3000000. / 1600.).human_throughput_bytes());
        assert_eq!("4.8B/m", (54327375. / 675876554.).human_throughput_bytes());
        assert_eq!("28.9B/h", (5432737. / 675876554.).human_throughput_bytes());
        assert_eq!("8B/s", (5432737542. / 675876554.).human_throughput_bytes());
        assert_eq!("1B/s", (1. / 0.99).human_throughput_bytes());
        assert_eq!("1B/s", (1. / 0.999).human_throughput_bytes());
        assert_eq!("1B/s", (1. / 1.00001).human_throughput_bytes());
        assert_eq!("1B/s", (1. / 1.0001).human_throughput_bytes());
        assert_eq!("9B/d", (125. / 1200000.).human_throughput_bytes());
        assert_eq!("9.5B/d", (132. / 1200000.).human_throughput_bytes());
        assert_eq!("9.72B/d", (135. / 1200000.).human_throughput_bytes());
        assert_eq!("1B/h", (1. / 3599.).human_throughput_bytes());
        assert_eq!("1B/h", (1. / 3600.).human_throughput_bytes());
        assert_eq!("23.99B/d", (1. / 3601.).human_throughput_bytes());
        assert_eq!("23.95B/d", (1. / 3608.).human_throughput_bytes());
        assert_eq!("2.16B/d", (2. / 80000.).human_throughput_bytes());
    }

    #[test]
    fn flexibility() {
        assert_eq!("123MCrabs/s", 123e6.human_throughput("Crabs"));
        assert_eq!("123MCrabs/s", 123e6.human_throughput("Crabs".to_owned()));
        assert_eq!("123M🦀/s", 123e6.human_throughput("🦀"));
    }

    #[test]
    fn ownership() {
        let mut a = 42000;
        assert_eq!("42kB/s", a.human_throughput_bytes());
        assert_eq!("42kB/s", (&a).human_throughput_bytes());
        assert_eq!("42kB/s", (&mut a).human_throughput_bytes());
    }
}
