mod human_count;
mod human_duration;
mod human_throughput;

const BYTES: &str = "B";

pub trait HumanRepr: sealed::Sealed + Sized {
    fn human_count(self, what: &str) -> String;
    fn human_count_bytes(self) -> String {
        self.human_count(BYTES)
    }

    fn human_duration(self) -> String;

    fn human_throughput(self, what: &str) -> String;
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
pub fn rounded(val: f64, dec: usize) -> f64 {
    match dec {
        1 => (val * 10.).round() / 10.,
        2 => (val * 100.).round() / 100.,
        // 0 => val.round(),
        _ => unreachable!(),
    }
}
