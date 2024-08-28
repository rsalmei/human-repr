use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{self, Display, Write};
use std::sync::{LazyLock, Mutex};

pub const BYTES: &str = "B";

/// Round a value to the given number of decimals.
pub fn rounded(val: f64, dec: u8) -> f64 {
    match dec {
        0 => val.round(),
        1 => (val * 10.).round() / 10.,
        2 => (val * 100.).round() / 100.,
        _ => {
            let pow = 10f64.powi(dec as _);
            (val * pow).round() / pow
        }
    }
}

/// Return the minimum number of decimals to display.
pub fn min_decimals(r: f64) -> usize {
    match r {
        _ if r.fract() < 0.005 => 0,
        _ if (r * 10.).fract() < 0.05 => 1,
        _ => 2,
    }
}

/// Intern a string to prevent duplicates and redundant allocations.
pub fn intern<'a>(text: impl Into<Cow<'a, str>>) -> &'static str {
    fn intern(text: Cow<str>) -> &'static str {
        static CACHE: LazyLock<Mutex<HashSet<&'static str>>> = LazyLock::new(Default::default);

        let mut cache = CACHE.lock().unwrap();
        match cache.get(text.as_ref()) {
            Some(x) => x,
            None => {
                let interned = Box::leak(text.into_owned().into_boxed_str());
                cache.insert(interned);
                interned
            }
        }
    }
    intern(text.into())
}

struct HeapLessCompare<'a, I>(&'a mut I);

impl<I: Iterator<Item = u8>> Write for HeapLessCompare<'_, I> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes().try_for_each(|c| match self.0.next() {
            Some(ex) if c == ex => Ok(()),
            _ => Err(fmt::Error),
        })
    }
}

pub fn compare_display(expected: &str, human: &impl Display) -> bool {
    let mut it = expected.bytes();
    write!(HeapLessCompare(it.by_ref()), "{human}").is_ok_and(|()| it.len() == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding() {
        assert_eq!(rounded(23456.23456, 0), 23456.);
        assert_eq!(rounded(23456.23456, 1), 23456.2);
        assert_eq!(rounded(23456.23456, 2), 23456.23);
        assert_eq!(rounded(23456.23456, 3), 23456.235);
        assert_eq!(rounded(23456.23456, 4), 23456.2346);
        assert_eq!(rounded(23456.23456, 5), 23456.23456);
        assert_eq!(rounded(23456.23456, u8::MAX), 23456.23456);
    }

    #[test]
    fn decimals() {
        assert_eq!(min_decimals(0.), 0);
        assert_eq!(min_decimals(1234.), 0);
        assert_eq!(min_decimals(1e30), 0);
        assert_eq!(min_decimals(0.001), 0);

        assert_eq!(min_decimals(0.1), 1);
        assert_eq!(min_decimals(1234.1), 1);
        assert_eq!(min_decimals(1234.9), 1);
        assert_eq!(min_decimals(4236784632786.9), 1);

        assert_eq!(min_decimals(0.005), 2);
        assert_eq!(min_decimals(0.05), 2);
        assert_eq!(min_decimals(0.01), 2);
        assert_eq!(min_decimals(1234.91), 2);
        assert_eq!(min_decimals(1234.99), 2);
        assert_eq!(min_decimals(4236784632786.99), 2);
        assert_eq!(min_decimals(1234.999), 2);
    }

    #[test]
    fn interning() {
        let s = "human";
        let s2 = intern(s);
        assert_eq!(s, s2);
        assert_ne!(s.as_ptr(), s2.as_ptr());
        let s3 = intern(s);
        assert_eq!(s, s3);
        assert_ne!(s.as_ptr(), s3.as_ptr());
        assert_eq!(s2.as_ptr(), s3.as_ptr());
    }

    #[test]
    fn heapless_compare() {
        assert!(compare_display("123", &123));
        assert!(!compare_display("12", &123));
        assert!(!compare_display("1234", &123));
    }
}
