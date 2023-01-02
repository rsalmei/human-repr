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
