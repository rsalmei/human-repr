mod human_count;
mod human_duration;
mod human_throughput;

const BYTES: &str = "B";

pub trait HumanRepr: sealed::Sealed + Sized {
    /// Generate a beautiful human count.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("43.2 Mcoins", 43214321u32.human_count("coins"));
    /// ```
    fn human_count(self, what: &str) -> String;
    /// Generate a beautiful human count.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("43.2 MB", 43214321u32.human_count_bytes());
    /// ```
    fn human_count_bytes(self) -> String {
        self.human_count(BYTES)
    }

    /// Generate a beautiful human duration.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("160 ms", 0.1599999.human_duration());
    /// ```
    fn human_duration(self) -> String;

    /// Generate a beautiful human throughput.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2 Mcoins/s", 1234567.8.human_throughput("coins"));
    /// ```
    fn human_throughput(self, what: &str) -> String;
    /// Generate a beautiful human throughput.
    ///
    /// ```
    /// use human_repr::HumanRepr;
    /// assert_eq!("1.2 MB/s", 1234567.8.human_throughput_bytes());
    /// ```
    fn human_throughput_bytes(self) -> String {
        self.human_throughput(BYTES)
    }
}

macro_rules! impl_human {
    {$($t:ty),+} => {$(
        impl HumanRepr for $t {
            fn human_count(self, what: &str) -> String {
                human_count::conv(self as f64, what)
            }
            fn human_duration(self) -> String {
                human_duration::conv(self as f64)
            }
            fn human_throughput(self, what: &str) -> String {
                human_throughput::conv(self as f64, what)
            }
        }
    )+}
}
impl_human!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);

mod sealed {
    pub trait Sealed {}
    macro_rules! impl_sealed {
        {$($t:ty),+} => {
            $(impl Sealed for $t {})+
        }
    }
    impl_sealed!(u8, u16, u32, u64, u128, usize, f32, f64, i8, i16, i32, i64, i128, isize);
}

const SPACE: &str = {
    match cfg!(feature = "nospace") {
        true => "",
        false => " ",
    }
};

#[inline]
fn rounded(val: f64, dec: usize) -> f64 {
    match dec {
        1 => (val * 10.).round() / 10.,
        2 => (val * 100.).round() / 100.,
        // 0 => val.round(),
        _ => unreachable!(),
    }
}
