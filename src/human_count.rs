use super::{rounded, SPACE};

// Not enabling any optional features gets: SI symbols, divisor is 1000, and with space.
const SPEC: &[&str] = {
    match cfg!(feature = "iec") {
        true => &["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"],
        false => &["", "k", "M", "G", "T", "P", "E", "Z", "Y"], // SI
    }
};
const DECIMALS: &[usize] = &[1, 1, 1, 2, 2, 2, 2, 2, 2];
const DIVISOR: f64 = {
    match cfg!(feature = "1024") {
        true => 1024.,
        false => 1000.,
    }
};

pub fn conv(mut val: f64, what: &str) -> String {
    for (&scale, &dec) in SPEC.iter().zip(DECIMALS) {
        match rounded(val, dec) {
            r if r.abs() >= DIVISOR => val /= DIVISOR,
            r if r.fract() == 0. => return format!("{r:.0}{SPACE}{scale}{what}"),
            r if (r * 10.).fract() == 0. => return format!("{r:.1}{SPACE}{scale}{what}"),
            r => return format!("{r:.0$}{SPACE}{scale}{what}", dec),
        }
    }

    format!("{val:.2}{SPACE}+{what}")
}

#[cfg(test)]
mod tests {
    use crate::HumanRepr;

    #[test]
    fn basic() {
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
}
