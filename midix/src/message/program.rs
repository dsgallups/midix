use crate::prelude::*;
use core::fmt;

/// Identifies an instrument
///
/// TODO docs
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Program(u8);

impl MidiBits for Program {
    type BitRepresentation = u8;
    fn as_bits(&self) -> Self::BitRepresentation {
        self.0
    }
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Self(check_u7(rep)?))
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
/// Identifies an instrument
///
/// TODO docs
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ProgramRef<'a>(&'a u8);

impl<'a> ProgramRef<'a> {
    /// Create a new program command. Does not check for u7.
    pub(crate) const fn new(program: &'a u8) -> Self {
        Self(program)
    }

    pub fn byte(&self) -> &u8 {
        self.0
    }
}
