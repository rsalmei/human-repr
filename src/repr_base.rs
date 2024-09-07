/// The system used to represent metric prefixes.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum System {
    /// SI system (1000 divisor).
    #[default]
    SI,
    /// SI system with binary prefix (1024 divisor).
    SI2,
    /// IEC system (1024 divisor).
    IEC,
}

/// The precision used when formatting values.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Precision {
    /// Automatically select the best precision, from 0 to 2 decimals.
    #[default]
    Auto,
    /// Use the given number of decimals.
    Fixed(u8),
    /// Use full precision.
    Full,
}

impl From<u8> for Precision {
    fn from(value: u8) -> Self {
        Self::Fixed(value)
    }
}

/// Specifies how the human representation should split values, prefixes, and units.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Split {
    /// Do not split at any point, i.e., `{value}{prefix}{unit}`.
    #[default]
    Join,
    /// Split at the prefix, i.e., `{value} {prefix}{unit}`.
    Prefix,
    /// Split at the unit, i.e., `{value}{prefix} {unit}`.
    Unit,
}

impl From<bool> for Split {
    fn from(value: bool) -> Self {
        Self::from(value)
    }
}

impl Split {
    pub const fn from(value: bool) -> Self {
        if value {
            Split::Prefix
        } else {
            Split::Join
        }
    }
}
