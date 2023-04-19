use super::HumanThroughputData;
use crate::utils::{self, SPACE};
use std::fmt::{self, Debug, Display};

const SPEC: &[(f64, &str, usize)] = &[
    (24., "/d", 2),
    (60., "/h", 1),
    (60., "/min", 1),
    // "/s" in code.
];

impl Display for HumanThroughputData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HumanThroughputData { mut val, unit } = self;
        val *= 60. * 60. * 24.;
        for &(size, scale, dec) in SPEC {
            match utils::rounded(val, dec) {
                r if r.abs() >= size => val /= size,
                r if r.fract() == 0. => return write!(f, "{:.0}{}{}{}", r, SPACE, unit, scale),
                r if (r * 10.).fract() == 0. => {
                    return write!(f, "{:.1}{}{}{}", r, SPACE, unit, scale)
                }
                r => return write!(f, "{:.2}{}{}{}", r, SPACE, unit, scale),
            }
        }

        use super::HumanCount;
        write!(f, "{}/s", val.human_count(unit.as_ref()))
    }
}

impl Debug for HumanThroughputData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("HumanThroughput");
        ds.field("val", &self.val);
        ds.field("unit", &self.unit);
        ds.finish()?;
        write!(f, " -> ")?;
        fmt::Display::fmt(self, f)
    }
}

impl PartialEq<HumanThroughputData<'_>> for &str {
    fn eq(&self, other: &HumanThroughputData<'_>) -> bool {
        utils::display_compare(self, other)
    }
}

impl PartialEq<&str> for HumanThroughputData<'_> {
    fn eq(&self, other: &&str) -> bool {
        other == self
    }
}

#[cfg(all(test, not(any(feature = "1024", feature = "iec", feature = "space"))))]
mod tests {
    use crate::HumanThroughput;

    #[test]
    fn operation() {
        assert_eq!("1B/s", 1.human_throughput_bytes());
        assert_eq!("-1B/s", (-1).human_throughput_bytes());
        assert_eq!("1.2MB/s", (1234567. / 1.).human_throughput_bytes());
        assert_eq!("10B/s", (10. / 1.).human_throughput_bytes());
        assert_eq!("30B/min", (1. / 2.).human_throughput_bytes());
        assert_eq!("-30B/min", (-1. / 2.).human_throughput_bytes());
        assert_eq!("5B/s", (10. / 2.).human_throughput_bytes());
        assert_eq!("5.5B/s", (11. / 2.).human_throughput_bytes());
        assert_eq!("6.1B/min", (10. / 99.).human_throughput_bytes());
        assert_eq!("1.8B/min", (3. / 100.).human_throughput_bytes());
        assert_eq!("4.4B/min", (8. / 110.).human_throughput_bytes());
        assert_eq!("6.8B/h", (3. / 1600.).human_throughput_bytes());
        assert_eq!("1.9kB/s", (3000000. / 1600.).human_throughput_bytes());
        assert_eq!("4.8B/min", (5432737. / 67587655.).human_throughput_bytes());
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
        assert_eq!("123MCrabs/s", 123e6.human_throughput(&"Crabs".to_owned()));
        assert_eq!("123M🦀/s", 123e6.human_throughput("🦀"));
        assert_eq!("12.3k°C/s", 123e2.human_throughput("°C"));
        assert_eq!("1.2°C/s", 123e-2.human_throughput("°C"));
    }

    #[test]
    #[allow(clippy::needless_borrow)]
    fn ownership() {
        let mut a = 42000;
        assert_eq!("42kB/s", a.human_throughput_bytes());
        assert_eq!("42kB/s", (&a).human_throughput_bytes());
        assert_eq!("42kB/s", (&mut a).human_throughput_bytes());
    }

    #[test]
    fn symmetric() {
        assert_eq!(1.human_throughput_bytes(), "1B/s");
    }
}

#[test]
#[cfg(feature = "serde")]
fn serialize() -> Result<(), serde_json::Error> {
    use crate::HumanThroughput;
    let h = 123456.human_throughput("X");
    let ser = serde_json::to_string(&h)?;
    assert_eq!(r#"{"val":123456.0,"unit":"X"}"#, &ser);
    let h2 = serde_json::from_str::<HumanThroughputData>(&ser)?;
    assert_eq!(h, h2);
    Ok(())
}
