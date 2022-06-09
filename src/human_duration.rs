use super::{rounded, HumanDuration, SPACE};
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
                r if r.fract() == 0. => return write!(f, "{r:.0}{SPACE}{scale}"),
                r if (r * 10.).fract() == 0. => return write!(f, "{r:.1}{SPACE}{scale}"),
                r => return write!(f, "{r:.dec$}{SPACE}{scale}"),
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

#[cfg(test)]
mod tests {
    use crate::HumanRepr;

    #[test]
    fn operation() {
        assert_eq!("1 s", 1.human_duration());
        assert_eq!("-1 s", -1.human_duration());
        assert_eq!("1.2 ns", 0.00000000123.human_duration());
        assert_eq!("1.8 ns", 0.0000000018.human_duration());
        assert_eq!("1.9 ns", 0.00000000185.human_duration());
        assert_eq!("1 µs", 0.000001.human_duration());
        assert_eq!("-1 µs", -0.000001.human_duration());
        assert_eq!("1 µs", 0.000000999996.human_duration());
        assert_eq!("10 µs", 0.00001.human_duration());
        assert_eq!("15.6 µs", 0.0000156.human_duration());
        assert_eq!("10 ms", 0.01.human_duration());
        assert_eq!("14.1 ms", 0.0141233333333.human_duration());
        assert_eq!("1 ms", 0.000999999.human_duration());
        assert_eq!("20 ms", 0.0199999.human_duration());
        assert_eq!("110 ms", 0.1099999.human_duration());
        assert_eq!("160 ms", 0.1599999.human_duration());
        assert_eq!("801.5 ms", 0.8015.human_duration());
        assert_eq!("3.43 s", 3.434999.human_duration());
        assert_eq!("3.44 s", 3.435999.human_duration());
        assert_eq!("59 s", 59.0.human_duration());
        assert_eq!("59.9 s", 59.9.human_duration());
        assert_eq!("59.99 s", 59.99.human_duration());
        assert_eq!("1:00", 59.995.human_duration());
        assert_eq!("1:08", 68.09.human_duration());
        assert_eq!("19:21", 1160.99.human_duration());
        assert_eq!("1:04:48", 3888.395.human_duration());
        assert_eq!("2:46:40", 10000u16.human_duration());
        assert_eq!("27:46:40", 100000i64.human_duration());
        assert_eq!("277:46:40", 1000000isize.human_duration());
    }

    #[test]
    fn ownership() {
        let mut a = 0.01;
        assert_eq!("10 ms", a.human_duration());
        assert_eq!("10 ms", (&a).human_duration());
        assert_eq!("10 ms", (&mut a).human_duration());
    }
}
