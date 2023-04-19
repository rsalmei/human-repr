use std::fmt;

pub const SPACE: &str = {
    match cfg!(feature = "space") {
        true => " ",
        false => "",
    }
};

#[inline]
pub fn rounded(val: f64, dec: usize) -> f64 {
    match dec {
        0 => val.round(),
        1 => (val * 10.).round() / 10.,
        2 => (val * 100.).round() / 100.,
        _ => unreachable!(),
    }
}

pub struct DisplayCompare<'a, I>(&'a mut I);

impl<I: Iterator<Item = u8>> fmt::Write for DisplayCompare<'_, I> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match s
            .bytes()
            .map(|c| (c, self.0.next()))
            .all(|(x, y)| x == y.unwrap_or_default())
        {
            true => Ok(()),
            false => Err(fmt::Error),
        }
    }
}

pub fn display_compare(expected: &str, human: &impl fmt::Display) -> bool {
    let mut it = expected.bytes();
    use fmt::Write;
    write!(DisplayCompare(it.by_ref()), "{human}").map_or(false, |_| it.len() == 0)
}
