use crate::metric_prefix::System;
use crate::repr_base::{Precision, Separator};
use crate::utils;
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

/// Representation options for Human Count.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct HumanCountRepr {
    pub unit: &'static str,
    pub sys: Option<System>,
    pub prec: Option<Precision>,
    pub sep: Option<Separator>,
}

/// This is used in the Debug impl of the [`super::HumanCountData`].
impl Display for HumanCountRepr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}", self.unit)?;
        if let Some(sys) = self.sys {
            write!(f, "_{sys:?}")?;
        }
        if let Some(prec) = self.prec {
            write!(f, ":{prec}")?;
        }
        if let Some(sep) = self.sep {
            write!(f, "-{sep:?}")?;
        }
        write!(f, ")")
    }
}

trait UpdateHumanCountRepr {
    fn update_repr(self, repr: &mut HumanCountRepr);
}

impl UpdateHumanCountRepr for &'static str {
    fn update_repr(self, repr: &mut HumanCountRepr) {
        repr.unit = self;
    }
}

impl UpdateHumanCountRepr for String {
    fn update_repr(self, repr: &mut HumanCountRepr) {
        repr.unit = utils::intern(self);
    }
}

impl UpdateHumanCountRepr for Cow<'_, str> {
    fn update_repr(self, repr: &mut HumanCountRepr) {
        repr.unit = utils::intern(self);
    }
}

impl UpdateHumanCountRepr for System {
    fn update_repr(self, repr: &mut HumanCountRepr) {
        repr.sys = Some(self);
    }
}

impl UpdateHumanCountRepr for u8 {
    fn update_repr(self, repr: &mut HumanCountRepr) {
        repr.prec = Some(Precision::from(self));
    }
}

impl UpdateHumanCountRepr for Precision {
    fn update_repr(self, repr: &mut HumanCountRepr) {
        repr.prec = Some(self);
    }
}

impl UpdateHumanCountRepr for bool {
    fn update_repr(self, repr: &mut HumanCountRepr) {
        repr.sep = Some(Separator::from(self));
    }
}

impl UpdateHumanCountRepr for Separator {
    fn update_repr(self, repr: &mut HumanCountRepr) {
        repr.sep = Some(self);
    }
}

pub trait IntoHumanCountRepr {
    fn into_repr(self) -> HumanCountRepr;
}

impl<T: UpdateHumanCountRepr> IntoHumanCountRepr for T {
    fn into_repr(self) -> HumanCountRepr {
        (self,).into_repr()
    }
}

impl<T1> IntoHumanCountRepr for (T1,)
where
    T1: UpdateHumanCountRepr,
{
    fn into_repr(self) -> HumanCountRepr {
        let mut repr = HumanCountRepr::default();
        self.0.update_repr(&mut repr);
        repr
    }
}

impl<T1, T2> IntoHumanCountRepr for (T1, T2)
where
    T1: UpdateHumanCountRepr,
    T2: UpdateHumanCountRepr,
{
    fn into_repr(self) -> HumanCountRepr {
        let mut repr = HumanCountRepr::default();
        self.0.update_repr(&mut repr);
        self.1.update_repr(&mut repr);
        repr
    }
}

impl<T1, T2, T3> IntoHumanCountRepr for (T1, T2, T3)
where
    T1: UpdateHumanCountRepr,
    T2: UpdateHumanCountRepr,
    T3: UpdateHumanCountRepr,
{
    fn into_repr(self) -> HumanCountRepr {
        let mut repr = HumanCountRepr::default();
        self.0.update_repr(&mut repr);
        self.1.update_repr(&mut repr);
        self.2.update_repr(&mut repr);
        repr
    }
}

impl<T1, T2, T3, T4> IntoHumanCountRepr for (T1, T2, T3, T4)
where
    T1: UpdateHumanCountRepr,
    T2: UpdateHumanCountRepr,
    T3: UpdateHumanCountRepr,
    T4: UpdateHumanCountRepr,
{
    fn into_repr(self) -> HumanCountRepr {
        let mut repr = HumanCountRepr::default();
        self.0.update_repr(&mut repr);
        self.1.update_repr(&mut repr);
        self.2.update_repr(&mut repr);
        self.3.update_repr(&mut repr);
        repr
    }
}

impl HumanCountRepr {
    pub fn new<'a, P: Into<Precision>, S: Into<Separator>>(
        unit: impl Into<Cow<'a, str>>,
        sys: impl Into<Option<System>>,
        prec: impl Into<Option<P>>,
        sep: impl Into<Option<S>>,
    ) -> Self {
        Self {
            unit: utils::intern(unit.into()),
            sys: sys.into(),
            prec: prec.into().map(Into::into),
            sep: sep.into().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_into_repr() {
        // this will generate exactly 446 test cases, with all possible permutations
        // of the parameters, including directly T and tuples with 1 to 4 elements.
        // asked on the Rust users forum: https://users.rust-lang.org/t/macro-to-generate-combinations-for-test-cases/116416
        macro_rules! base {
            ($repr:expr => $unit:expr, $sys:expr, $prec:expr, $sep:expr) => {
                assert_eq!(
                    // HumanCountRepr::new($unit, $sys, $prec, $sep),
                    HumanCountRepr {
                        unit: $unit.into(),
                        sys: $sys.into(),
                        prec: $prec.into(),
                        sep: $sep.into(),
                    },
                    $repr.into_repr()
                );
            };
        }

        macro_rules! permute {
            // main recursion
            ([$current:expr, $($input:expr,)*] [$($prefix:expr,)*] /* insert here or later */ [$head:expr, $($tail:expr,)*] => $($p:expr),*) => {
                // "insert here"
                // leave 'prefix' empty for recursive call to allow all positions for next 'current'
                permute!([$($input,)*] [] /* next: insert here or later */ [$($prefix,)* $current, $head, $($tail,)*] => $($p),*);

                // "or later"
                // move 'head' over, allows all remaining positions recursively
                permute!([$current, $($input,)*] [$($prefix,)* $head,] /* next: insert here or later */ [$($tail,)*] => $($p),*);
            };
            // simplified case when tail is empty
            ([$current:expr, $($input:expr,)*] [$($prefix:expr,)*] /* insert here */ [] => $($p:expr),*) => {
                permute!([$($input,)*] [] [$($prefix,)* $current,] => $($p),*);
            };

            // when we're done
            ([][][$($a:expr,)*] => $($p:expr),*) => {
                base!(($($a),*) => $($p),*);
            }
        }

        macro_rules! case {
            // initial step
            (($($t:tt)*) => $($p:expr),*) => {
                case!(@[] ($($t)*) => $($p),*);
            };
            ($(@$i:ident)? $a:expr => $($p:expr),*) => {
                case!(@[] ($(@$i)?$a) => $($p),*);
            };

            // process each entry
            (@[$($processed:expr,)*] (@unit $a:expr $(, $($more:tt)*)?) => $($p:expr),*) => {
                case!(@[$($processed,)* $a              ,] ($($($more)*)?) => $($p),*);
                case!(@[$($processed,)* String::from($a),] ($($($more)*)?) => $($p),*);
                case!(@[$($processed,)* Cow::from($a)   ,] ($($($more)*)?) => $($p),*);
            };
            (@[$($processed:expr,)*] (@prec $a:expr $(, $($more:tt)*)?) => $($p:expr),*) => {
                case!(@[$($processed,)* $a                 ,] ($($($more)*)?) => $($p),*);
                case!(@[$($processed,)* Precision::from($a),] ($($($more)*)?) => $($p),*);
            };
            (@[$($processed:expr,)*] (@sep $a:expr $(, $($more:tt)*)?) => $($p:expr),*) => {
                case!(@[$($processed,)* $a                 ,] ($($($more)*)?) => $($p),*);
                case!(@[$($processed,)* Separator::from($a),] ($($($more)*)?) => $($p),*);
            };
            (@[$($processed:expr,)*] ($a:expr $(, $($more:tt)*)?) => $($p:expr),*) => {
                case!(@[$($processed,)* $a              ,] ($($($more)*)?) => $($p),*);
            };

            // final step
            (@[$a:expr,] () => $($p:expr),*) => {
                base!($a => $($p),*);
                base!(($a,) => $($p),*);
            };
            (@[$($a:expr,)*] () => $($p:expr),*) => {
                permute!([$($a,)*][][] => $($p),*);
            };
        }

        // --- one argument.
        case!(@unit "U" => "U", None, None, None);
        case!(System::SI2 => "", System::SI2, None, None);
        case!(@prec 5 => "", None, Precision::Fixed(5), None);
        case!(@sep false => "", None, None, Separator::No);

        // --- two arguments.
        case!((@unit "U", System::SI2) => "U", System::SI2, None, None);
        case!((@unit "U", @prec 5) => "U", None, Precision::Fixed(5), None);
        case!((@unit "U", @sep false) => "U", None, None, Separator::No);
        case!((System::SI2, @prec 5) => "", System::SI2, Precision::Fixed(5), None);
        case!((System::SI2, @sep false) => "", System::SI2, None, Separator::No);
        case!((@prec 5, @sep false) => "", None, Precision::Fixed(5), Separator::No);

        // --- three arguments.
        case!((@unit "U", System::SI2, @prec 5) => "U", System::SI2, Precision::Fixed(5), None);
        case!((@unit "U", System::SI2, @sep false) => "U", System::SI2, None, Separator::No);
        case!((System::SI2, @prec 5, @sep false) => "", System::SI2, Precision::Fixed(5), Separator::No);

        // --- four arguments.
        case!((@unit "U", System::SI2, @prec 5, @sep false) => "U", System::SI2, Precision::Fixed(5), Separator::No);
    }
}
