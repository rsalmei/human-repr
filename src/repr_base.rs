use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precision {
    Fixed(u8),
    Full,
}

impl Display for Precision {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Precision::Fixed(prec) => write!(f, "{prec}"),
            Precision::Full => write!(f, "full"),
        }
    }
}

impl From<u8> for Precision {
    fn from(value: u8) -> Self {
        Self::Fixed(value)
    }
}

/// Tells if the human representation should separate numbers and prefixes.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Separator {
    Yes,
    No,
}

impl Display for Separator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Separator::Yes => write!(f, " "),
            Separator::No => Ok(()),
        }
    }
}

impl From<bool> for Separator {
    fn from(value: bool) -> Self {
        Self::from(value)
    }
}

impl Separator {
    pub const fn from(value: bool) -> Self {
        if value {
            Separator::Yes
        } else {
            Separator::No
        }
    }
}
