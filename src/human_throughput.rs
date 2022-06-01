use super::{rounded, HumanRepr, SPACE};

const SPEC: &[(f64, &str, usize)] = &[
    (24., "/d", 2),
    (60., "/h", 1),
    (60., "/m", 1),
    // "/s" in code.
];

pub fn conv(mut val: f64, what: &str) -> String {
    val *= 60. * 60. * 24.;
    for &(size, scale, dec) in SPEC {
        match rounded(val, dec) {
            r if r.abs() >= size => val /= size,
            r if r.fract() == 0. => return format!("{r:.0}{SPACE}{what}{scale}"),
            r if (r * 10.).fract() == 0. => return format!("{r:.1}{SPACE}{what}{scale}"),
            r => return format!("{r:.0$}{SPACE}{what}{scale}", dec),
        }
    }

    format!("{}/s", val.human_count(what))
}

#[cfg(test)]
mod tests {
    use crate::HumanRepr;

    #[test]
    fn operation() {
        assert_eq!("1 B/s", 1.human_throughput_bytes());
        assert_eq!("-1 B/s", (-1.).human_throughput_bytes());
        assert_eq!("1.2 MB/s", (1234567. / 1.).human_throughput_bytes());
        assert_eq!("10 B/s", (10. / 1.).human_throughput_bytes());
        assert_eq!("30 B/m", (1. / 2.).human_throughput_bytes());
        assert_eq!("-30 B/m", (-1. / 2.).human_throughput_bytes());
        assert_eq!("5 B/s", (10. / 2.).human_throughput_bytes());
        assert_eq!("5.5 B/s", (11. / 2.).human_throughput_bytes());
        assert_eq!("6.1 B/m", (10. / 99.).human_throughput_bytes());
        assert_eq!("1.8 B/m", (3. / 100.).human_throughput_bytes());
        assert_eq!("4.4 B/m", (8. / 110.).human_throughput_bytes());
        assert_eq!("6.8 B/h", (3. / 1600.).human_throughput_bytes());
        assert_eq!("1.9 kB/s", (3000000. / 1600.).human_throughput_bytes());
        assert_eq!("4.8 B/m", (54327375. / 675876554.).human_throughput_bytes());
        assert_eq!("28.9 B/h", (5432737. / 675876554.).human_throughput_bytes());
        assert_eq!("8 B/s", (5432737542. / 675876554.).human_throughput_bytes());
        assert_eq!("1 B/s", (1. / 0.99).human_throughput_bytes());
        assert_eq!("1 B/s", (1. / 0.999).human_throughput_bytes());
        assert_eq!("1 B/s", (1. / 1.00001).human_throughput_bytes());
        assert_eq!("1 B/s", (1. / 1.0001).human_throughput_bytes());
        assert_eq!("9 B/d", (125. / 1200000.).human_throughput_bytes());
        assert_eq!("9.5 B/d", (132. / 1200000.).human_throughput_bytes());
        assert_eq!("9.72 B/d", (135. / 1200000.).human_throughput_bytes());
        assert_eq!("1 B/h", (1. / 3599.).human_throughput_bytes());
        assert_eq!("1 B/h", (1. / 3600.).human_throughput_bytes());
        assert_eq!("23.99 B/d", (1. / 3601.).human_throughput_bytes());
        assert_eq!("23.95 B/d", (1. / 3608.).human_throughput_bytes());
        assert_eq!("2.16 B/d", (2. / 80000.).human_throughput_bytes());
    }

    #[test]
    fn flexibility() {
        assert_eq!("123 MCrabs/s", 123e6.human_throughput("Crabs"));
        assert_eq!("123 MCrabs/s", 123e6.human_throughput("Crabs".to_owned()));
        assert_eq!("123 M🦀/s", 123e6.human_throughput("🦀"));
    }

    #[test]
    fn ownership() {
        let mut a = 42000;
        assert_eq!("42 kB/s", a.human_throughput_bytes());
        assert_eq!("42 kB/s", (&a).human_throughput_bytes());
        assert_eq!("42 kB/s", (&mut a).human_throughput_bytes());
    }
}
