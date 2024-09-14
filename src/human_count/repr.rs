use crate::{utils, Precision, Split, System};
use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::RwLock;

/// The HumanCount representation options.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HumanCountRepr {
    pub unit: &'static str,
    pub sys: Option<System>,
    pub prec: Option<Precision>,
    pub split: Option<Split>,
}

/// This is used in the Debug impl of the [`HumanCountData`].
impl Display for HumanCountRepr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let sep = |f: &mut Formatter<'_>, previous| {
            if previous {
                write!(f, "+")?;
            }
            Ok::<_, fmt::Error>(())
        };
        let mut previous = false;
        if !self.unit.is_empty() {
            write!(f, "{:?}", self.unit)?;
            previous = true;
        }
        if let Some(sys) = self.sys {
            sep(f, previous)?;
            write!(f, "{sys:?}")?;
            previous = true;
        }
        if let Some(prec) = self.prec {
            sep(f, previous)?;
            write!(f, "{prec:?}")?;
            previous = true;
        }
        if let Some(split) = self.split {
            sep(f, previous)?;
            write!(f, "{split:?}")?;
            previous = true;
        }
        if !previous {
            write!(f, "()")?;
        }
        Ok(())
    }
}

trait HumanCountReprParts {
    fn assign_into(self, repr: &mut HumanCountRepr);
}

impl HumanCountReprParts for &'static str {
    fn assign_into(self, repr: &mut HumanCountRepr) {
        repr.unit = self;
    }
}

impl HumanCountReprParts for String {
    fn assign_into(self, repr: &mut HumanCountRepr) {
        repr.unit = utils::intern(self);
    }
}

impl HumanCountReprParts for Cow<'_, str> {
    fn assign_into(self, repr: &mut HumanCountRepr) {
        repr.unit = utils::intern(self);
    }
}

impl HumanCountReprParts for System {
    fn assign_into(self, repr: &mut HumanCountRepr) {
        repr.sys = Some(self);
    }
}

impl HumanCountReprParts for u8 {
    fn assign_into(self, repr: &mut HumanCountRepr) {
        repr.prec = Some(Precision::from(self));
    }
}

impl HumanCountReprParts for Precision {
    fn assign_into(self, repr: &mut HumanCountRepr) {
        repr.prec = Some(self);
    }
}

impl HumanCountReprParts for bool {
    fn assign_into(self, repr: &mut HumanCountRepr) {
        repr.split = Some(Split::from(self));
    }
}

impl HumanCountReprParts for Split {
    fn assign_into(self, repr: &mut HumanCountRepr) {
        repr.split = Some(self);
    }
}

pub trait IntoHumanCountRepr {
    fn into_repr(self) -> HumanCountRepr;
}

impl<T: HumanCountReprParts> IntoHumanCountRepr for T {
    fn into_repr(self) -> HumanCountRepr {
        (self,).into_repr()
    }
}

impl IntoHumanCountRepr for () {
    fn into_repr(self) -> HumanCountRepr {
        HumanCountRepr::default()
    }
}

impl<T1> IntoHumanCountRepr for (T1,)
where
    T1: HumanCountReprParts,
{
    fn into_repr(self) -> HumanCountRepr {
        let mut repr = HumanCountRepr::default();
        self.0.assign_into(&mut repr);
        repr
    }
}

impl<T1, T2> IntoHumanCountRepr for (T1, T2)
where
    T1: HumanCountReprParts,
    T2: HumanCountReprParts,
{
    fn into_repr(self) -> HumanCountRepr {
        let mut repr = HumanCountRepr::default();
        self.0.assign_into(&mut repr);
        self.1.assign_into(&mut repr);
        repr
    }
}

impl<T1, T2, T3> IntoHumanCountRepr for (T1, T2, T3)
where
    T1: HumanCountReprParts,
    T2: HumanCountReprParts,
    T3: HumanCountReprParts,
{
    fn into_repr(self) -> HumanCountRepr {
        let mut repr = HumanCountRepr::default();
        self.0.assign_into(&mut repr);
        self.1.assign_into(&mut repr);
        self.2.assign_into(&mut repr);
        repr
    }
}

impl<T1, T2, T3, T4> IntoHumanCountRepr for (T1, T2, T3, T4)
where
    T1: HumanCountReprParts,
    T2: HumanCountReprParts,
    T3: HumanCountReprParts,
    T4: HumanCountReprParts,
{
    fn into_repr(self) -> HumanCountRepr {
        let mut repr = HumanCountRepr::default();
        self.0.assign_into(&mut repr);
        self.1.assign_into(&mut repr);
        self.2.assign_into(&mut repr);
        self.3.assign_into(&mut repr);
        repr
    }
}

static COUNT_SYS: RwLock<System> = RwLock::new(System::SI);
static COUNT_PREC: RwLock<Precision> = RwLock::new(Precision::Auto);
static COUNT_SPLIT: RwLock<Split> = RwLock::new(Split::Join);

impl HumanCountRepr {
    /// Create a new default HumanCount representation. Can be used as a builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the unit of this HumanCount representation.
    pub fn unit(mut self, unit: impl Into<Cow<'static, str>>) -> Self {
        self.unit = utils::intern(unit);
        self
    }

    /// Set the system of this HumanCount representation.
    pub fn system(mut self, sys: impl Into<Option<System>>) -> Self {
        self.sys = sys.into();
        self
    }

    /// Set the precision of this HumanCount representation.
    pub fn precision<P: Into<Precision>>(mut self, prec: impl Into<Option<P>>) -> Self {
        self.prec = prec.into().map(Into::into);
        self
    }

    /// Set the Split of this HumanCount representation.
    pub fn split<S: Into<Split>>(mut self, split: impl Into<Option<S>>) -> Self {
        self.split = split.into().map(Into::into);
        self
    }

    /// Create a new HumanCount representation with the given parts.
    pub fn with_parts<'a>(
        unit: impl Into<Cow<'a, str>>,
        sys: impl Into<Option<System>>,
        prec: impl Into<Option<Precision>>,
        split: impl Into<Option<Split>>,
    ) -> Self {
        Self {
            unit: utils::intern(unit),
            sys: sys.into(),
            prec: prec.into().map(Into::into),
            split: split.into().map(Into::into),
        }
    }

    /// Set the default System used for HumanCount.
    pub fn default_system(sys: System) {
        *COUNT_SYS.write().unwrap() = sys
    }

    /// Set the default precision used for HumanCount.
    pub fn default_precision(prec: Precision) {
        *COUNT_PREC.write().unwrap() = prec
    }

    /// Set the default Split used for HumanCount.
    pub fn default_split(split: Split) {
        *COUNT_SPLIT.write().unwrap() = split
    }

    /// Resolve the options from this HumanCount representation with the global settings.
    pub(super) fn resolve(&self) -> (&'static str, System, Precision, Split) {
        (
            self.unit,
            self.sys.unwrap_or_else(|| *COUNT_SYS.read().unwrap()),
            self.prec.unwrap_or_else(|| *COUNT_PREC.read().unwrap()),
            self.split.unwrap_or_else(|| *COUNT_SPLIT.read().unwrap()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        let repr = HumanCountRepr {
            unit: "X",
            sys: Some(System::IEC),
            prec: Some(Precision::Fixed(1)),
            split: Some(Split::Prefix),
        };
        assert_eq!(
            repr,
            HumanCountRepr::new()
                .unit("X")
                .system(System::IEC)
                .precision(1)
                .split(true)
        );
        assert_eq!(
            repr,
            HumanCountRepr::with_parts("X", System::IEC, Precision::Fixed(1), Split::Prefix)
        );
    }

    #[test]
    fn magic_into_repr() {
        // this will generate exactly 446 test cases, with all possible permutations
        // of the parameters, including directly T and tuples with 1 to 4 elements.
        // asked on the Rust users forum: https://users.rust-lang.org/t/macro-to-generate-combinations-for-test-cases/116416
        macro_rules! base {
            ($repr:expr => $unit:expr, $sys:expr, $prec:expr, $split:expr) => {
                assert_eq!(
                    HumanCountRepr::with_parts($unit, $sys, $prec, $split),
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
            (@[$($processed:expr,)*] (@split $a:expr $(, $($more:tt)*)?) => $($p:expr),*) => {
                case!(@[$($processed,)* $a                 ,] ($($($more)*)?) => $($p),*);
                case!(@[$($processed,)* Split::from($a),] ($($($more)*)?) => $($p),*);
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
        case!(() => "", None, None, None);
        case!(@unit "U" => "U", None, None, None);
        case!(System::SI2 => "", System::SI2, None, None);
        case!(@prec 5 => "", None, Precision::Fixed(5), None);
        case!(@split false => "", None, None, Split::Join);

        // --- two arguments.
        case!((@unit "U", System::SI2) => "U", System::SI2, None, None);
        case!((@unit "U", @prec 5) => "U", None, Precision::Fixed(5), None);
        case!((@unit "U", @split false) => "U", None, None, Split::Join);
        case!((System::SI2, @prec 5) => "", System::SI2, Precision::Fixed(5), None);
        case!((System::SI2, @split false) => "", System::SI2, None, Split::Join);
        case!((@prec 5, @split false) => "", None, Precision::Fixed(5), Split::Join);

        // --- three arguments.
        case!((@unit "U", System::SI2, @prec 5) => "U", System::SI2, Precision::Fixed(5), None);
        case!((@unit "U", System::SI2, @split false) => "U", System::SI2, None, Split::Join);
        case!((System::SI2, @prec 5, @split false) => "", System::SI2, Precision::Fixed(5), Split::Join);

        // --- four arguments.
        case!((@unit "U", System::SI2, @prec 5, @split false) => "U", System::SI2, Precision::Fixed(5), Split::Join);
    }
}
