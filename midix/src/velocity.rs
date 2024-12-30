use crate::prelude::*;
use core::fmt;
use std::borrow::Cow;

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
///
/// TODO
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Velocity<'a>(Cow<'a, u8>);

impl<'a> Velocity<'a> {
    /// Creates a new velocity from the provided byte
    ///
    /// Does not check for correctness
    pub const fn new(velocity: u8) -> Self {
        Self(Cow::Owned(velocity))
    }
    /// Create a new velocity from the referenced byte
    ///
    /// Does not check for correctness
    pub(crate) const fn new_borrowed(velocity: &'a u8) -> Self {
        Self(Cow::Borrowed(velocity))
    }
    /// Creates a new velocity from the provided byte
    ///
    /// Checks for correctness
    pub fn new_checked(rep: u8) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Self(Cow::Owned(check_u7(rep)?)))
    }

    /// Get a reference to the underlying byte
    pub fn byte(&self) -> &u8 {
        &self.0
    }

    /// Get a reference to the underlying byte
    pub fn value(&self) -> u8 {
        *self.0
    }
}

impl fmt::Display for Velocity<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
