use crate::prelude::*;
use core::fmt;

/// Identifies an instrument
///
/// TODO docs
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Program(DataByte);

impl Program {
    /// Creates a new program command.
    pub fn new<B>(rep: B) -> Result<Self, ParseError>
    where
        B: TryInto<DataByte, Error = ParseError>,
    {
        rep.try_into().map(Self)
    }

    /// Creates a new program command.
    ///
    /// Does not check that the byte is valid!
    #[inline]
    pub const fn new_unchecked(byte: u8) -> Self {
        Self(DataByte::new_unchecked(byte))
    }

    /// Get a reference to the underlying byte for the program.
    #[inline]
    pub const fn byte(&self) -> u8 {
        self.0.0
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
