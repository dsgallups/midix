use crate::prelude::*;
use core::fmt;
use std::borrow::Cow;

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
///
/// TODO
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Velocity<'a>(Cow<'a, u8>);

impl<'a> Velocity<'a> {
    pub const fn new(velocity: u8) -> Self {
        Self(Cow::Owned(velocity))
    }
    /// Create a new velocity
    pub(crate) const fn new_borrowed(velocity: &'a u8) -> Self {
        Self(Cow::Borrowed(velocity))
    }
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl MidiBits for Velocity<'_> {
    type BitRepresentation = u8;
    fn as_bits(&self) -> Self::BitRepresentation {
        *self.0
    }
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Self(Cow::Owned(check_u7(rep)?)))
    }
}

impl fmt::Display for Velocity<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
