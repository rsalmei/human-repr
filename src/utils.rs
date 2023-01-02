use std::fmt::{self, Write};

pub const SPACE: &str = {
    match cfg!(feature = "space") {
        true => " ",
        false => "",
    }
};

#[inline]
pub fn rounded(val: f64, dec: i8) -> f64 {
    let pow = 10f64.powi(dec as _);
    (val * pow).round() / pow
}

pub struct DisplayCompare<'a, I>(&'a mut I);

impl<I: Iterator<Item = u8>> Write for DisplayCompare<'_, I> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match s.bytes().zip(self.0.by_ref()).all(|(x, y)| x == y) {
            true => Ok(()),
            false => Err(fmt::Error),
        }
    }
}

pub fn display_compare(str: &str, display: &impl fmt::Display) -> bool {
    let mut it = str.bytes();
    write!(DisplayCompare(it.by_ref()), "{display}").map_or(false, |_| it.len() == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding() {
        assert_eq!(23456., rounded(23456.23456, 0));
        assert_eq!(23456.2, rounded(23456.23456, 1));
        assert_eq!(23456.23, rounded(23456.23456, 2));
        assert_eq!(23456.235, rounded(23456.23456, 3));
        assert_eq!(23456.2346, rounded(23456.23456, 4));
        assert_eq!(23456.23456, rounded(23456.23456, 5));
        assert_eq!(23460., rounded(23456.23456, -1));
        assert_eq!(23500., rounded(23456.23456, -2));
        assert_eq!(23000., rounded(23456.23456, -3));
        assert_eq!(20000., rounded(23456.23456, -4));
        assert_eq!(0., rounded(23456.23456, -5));
        assert_eq!(23456.23456, rounded(23456.23456, i8::MAX));
        assert_eq!(0., rounded(23456.23456, i8::MIN));
    }
}
