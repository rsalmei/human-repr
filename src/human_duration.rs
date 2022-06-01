use super::{rounded, SPACE};

const SPEC: &[(f64, f64, &str, usize)] = &[
    (1e3, 1e3, "ns", 1),
    (1e3, 1e3, "µs", 1), // uses non-ASCII “µs” suffix.
    (1e3, 1e3, "ms", 1),
    (60., 1., "s", 2),
    // 00:01:00 and beyond in code.
];

pub fn conv(mut val: f64) -> String {
    val *= 1e9;
    for &(size, next, scale, dec) in SPEC {
        match rounded(val, dec) {
            r if r.abs() >= size => val /= next,
            r if r.fract() == 0. => return format!("{r:.0}{SPACE}{scale}"),
            r if (r * 10.).fract() == 0. => return format!("{r:.1}{SPACE}{scale}"),
            r => return format!("{r:.0$}{SPACE}{scale}", dec),
        }
    }

    val = val.round();
    let m = val / 60.;
    format!(
        "{}:{:02}:{:02}",
        (m / 60.).trunc(),
        (m % 60.).trunc(),
        (val % 60.).trunc()
    )
}

#[cfg(test)]
mod tests {
    use crate::HumanRepr;

    #[test]
    fn operation() {
        assert_eq!("1 s", 1.human_duration());
        assert_eq!("-1 s", (-1).human_duration());
        assert_eq!("1.2 ns", 0.00000000123.human_duration());
        assert_eq!("1.8 ns", 0.0000000018.human_duration());
        assert_eq!("1.9 ns", 0.00000000185.human_duration());
        assert_eq!("1 µs", 0.000001.human_duration());
        assert_eq!("-1 µs", (-0.000001).human_duration());
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
        assert_eq!("0:01:00", 59.999.human_duration());
        assert_eq!("0:01:00", 60.0.human_duration());
        assert_eq!("0:01:08", 68.09.human_duration());
        assert_eq!("0:01:09", 68.5.human_duration());
        assert_eq!("0:01:01", 60.99.human_duration());
        assert_eq!("0:02:06", 125.825.human_duration());
        assert_eq!("1:14:48", 4488.395.human_duration());
        assert_eq!("2:46:40", 10000u64.human_duration());
        assert_eq!("27:46:40", 100000u64.human_duration());
    }

    #[test]
    fn ownership() {
        let mut a = 0.01;
        assert_eq!("10 ms", a.human_duration());
        assert_eq!("10 ms", (&a).human_duration());
        assert_eq!("10 ms", (&mut a).human_duration());
    }
}
