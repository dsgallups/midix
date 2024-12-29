use crate::prelude::*;
use core::fmt;
use std::borrow::Cow;

/// Identifies an instrument
///
/// TODO docs
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Program<'a>(Cow<'a, u8>);

impl<'a> Program<'a> {
    /// Creates a new program command.
    ///
    /// Does not check for correctness.
    pub const fn new(program: u8) -> Self {
        Self(Cow::Owned(program))
    }
    /// Create a new program command.
    ///
    /// Does not check for correctness.
    pub(crate) const fn new_borrowed(program: &'a u8) -> Self {
        Self(Cow::Borrowed(program))
    }

    /// Get a reference to the underlying byte for the program.
    pub fn byte(&self) -> &u8 {
        self.0.as_ref()
    }
}

impl MidiBits for Program<'_> {
    type BitRepresentation = u8;
    fn as_bits(&self) -> Self::BitRepresentation {
        *self.0
    }
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Self::new(check_u7(rep)?))
    }
}

impl fmt::Display for Program<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
