use super::{rounded, HumanDuration, HumanRepr, HumanReprDuration, SPACE};
use std::time::Duration;
use std::{fmt, ops};

const SPEC: &[(f64, f64, &str, usize)] = &[
    (1e3, 1e3, "ns", 1),
    (1e3, 1e3, "µs", 1), // uses non-ASCII “µs” suffix.
    (1e3, 1e3, "ms", 1),
    (60., 1., "s", 2),
    // 1:01.1 (minutes in code, 1 decimal).
    // 1:01:01 (hours in code, 0 decimal).
];

impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut val = self.0 * 1e9;
        for &(size, next, scale, dec) in SPEC {
            match rounded(val, dec) {
                r if r.abs() >= size => val /= next,
                r if r.fract() == 0. => return write!(f, "{:.0}{}{}", r, SPACE, scale),
                r if (r * 10.).fract() == 0. => return write!(f, "{:.1}{}{}", r, SPACE, scale),
                r => return write!(f, "{:.dec$}{}{}", r, SPACE, scale, dec = dec),
            }
        }

        val = rounded(val, 1);
        let m = val / 60.;
        match m < 60. {
            true => match val % 60. {
                s if s.fract() == 0. => write!(f, "{}:{:02}", m.trunc(), s.trunc()),
                s => write!(f, "{}:{:04}", m.trunc(), rounded(s, 1)),
            },
            false => write!(
                f,
                "{}:{:02}:{:02}",
                (m / 60.).trunc(),
                (m % 60.).trunc(),
                (val % 60.).trunc()
            ),
        }
    }
}

impl fmt::Debug for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HumanDuration")
            .field("val", &self.0)
            .finish()?;
        write!(f, " -> ")?;
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl PartialEq<HumanDuration> for &str {
    fn eq(&self, other: &HumanDuration) -> bool {
        *self == &other.to_string()
    }
}

impl PartialEq<&str> for HumanDuration {
    fn eq(&self, other: &&str) -> bool {
        other == &self.to_string()
    }
}

impl ops::Neg for HumanDuration {
    type Output = HumanDuration;

    fn neg(self) -> Self::Output {
        HumanDuration(-self.0)
    }
}

impl super::sealed::Sealed for Duration {}

impl HumanReprDuration for Duration {
    fn human_duration(self) -> HumanDuration {
        self.as_secs_f64().human_duration()
    }
}

#[cfg(test)]
mod tests {
    use crate::{HumanRepr, HumanReprDuration};
    use std::time::Duration;

    #[test]
    fn operation() {
        assert_eq!("1s", 1.human_duration());
        assert_eq!("-1s", -1.human_duration());
        assert_eq!("1ns", 0.00000000123.human_duration());
        assert_eq!("2ns", 0.0000000018.human_duration());
        assert_eq!("1µs", 0.000001.human_duration());
        assert_eq!("-1µs", -0.000001.human_duration());
        assert_eq!("1µs", 0.000000999996.human_duration());
        assert_eq!("10µs", 0.00001.human_duration());
        assert_eq!("15.6µs", 0.0000156.human_duration());
        assert_eq!("10ms", 0.01.human_duration());
        assert_eq!("14.1ms", 0.0141233333333.human_duration());
        assert_eq!("1ms", 0.000999999.human_duration());
        assert_eq!("20ms", 0.0199999.human_duration());
        assert_eq!("110ms", 0.1099999.human_duration());
        assert_eq!("160ms", 0.1599999.human_duration());
        assert_eq!("801.5ms", 0.8015.human_duration());
        assert_eq!("3.43s", 3.434999.human_duration());
        assert_eq!("3.44s", 3.435999.human_duration());
        assert_eq!("59s", 59.0.human_duration());
        assert_eq!("59.9s", 59.9.human_duration());
        assert_eq!("59.99s", 59.99.human_duration());
        assert_eq!("1:00", 59.995.human_duration());
        assert_eq!("1:08.1", 68.09.human_duration());
        assert_eq!("2:05.8", 125.825.human_duration());
        assert_eq!("19:20.4", 1160.36.human_duration());
        assert_eq!("1:04:48", 3888.395.human_duration());
        assert_eq!("2:46:40", 10000u16.human_duration());
        assert_eq!("27:46:40", 100000i64.human_duration());
        assert_eq!("277:46:40", 1000000isize.human_duration());
    }

    #[test]
    fn flexibility() {
        macro_rules! d {
            {$f:literal} => {
                Duration::from_secs_f64($f)
            };
            {$s:literal, $n:literal} => {
                Duration::new($s, $n)
            };
        }

        assert_eq!("1s", d!(1.).human_duration());
        assert_eq!("1.5s", d!(1.5).human_duration());
        assert_eq!("1ns", d!(0.00000000123).human_duration());
        assert_eq!("1ns", d!(0.00000000185).human_duration());
        assert_eq!("1ns", d!(0, 1).human_duration());
        assert_eq!("999ns", d!(0.000000999999999).human_duration());
        assert_eq!("1µs", d!(0, 1000).human_duration());
        assert_eq!("10µs", d!(0, 10000).human_duration());
        assert_eq!("15.6µs", d!(0, 15600).human_duration());
        assert_eq!("10ms", d!(0.01).human_duration());
        assert_eq!("14.1ms", d!(0.0141233333333).human_duration());
        assert_eq!("110ms", d!(0, 110000000).human_duration());
        assert_eq!("801.5ms", d!(0.8015).human_duration());
        assert_eq!("3.43s", d!(3.434999).human_duration());
        assert_eq!("59s", d!(59.0).human_duration());
        assert_eq!("59.9s", d!(59.9).human_duration());
        assert_eq!("59.99s", d!(59.99).human_duration());
        assert_eq!("1:00", d!(60, 0).human_duration());
        assert_eq!("1:08.1", d!(68.09).human_duration());
        assert_eq!("19:20.4", d!(1160, 350000000).human_duration());
        assert_eq!("1:04:48", d!(3888.395).human_duration());
        assert_eq!("2:46:40", d!(10000.).human_duration());
        assert_eq!("27:46:40", d!(100000.).human_duration());
        assert_eq!("277:46:40", d!(1000000, 1).human_duration());
    }

    #[test]
    fn ownership() {
        let mut a = 0.01;
        assert_eq!("10ms", a.human_duration());
        assert_eq!("10ms", (&a).human_duration());
        assert_eq!("10ms", (&mut a).human_duration());
    }
}
