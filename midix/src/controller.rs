use crate::prelude::*;
use core::fmt;
use std::borrow::Cow;

/// Identifies a device
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Controller<'a>(Cow<'a, u8>);

impl<'a> Controller<'a> {
    /// Interpret a byte as a type of device
    ///
    /// Checks for correctness (leading 0 bit)
    pub fn new(byte: u8) -> Result<Self, io::Error> {
        Ok(Self(Cow::Owned(check_u7(byte)?)))
    }

    /// Interpret a byte as a type of device
    ///
    /// Does not check for correctness
    pub const fn new_unchecked(byte: u8) -> Self {
        Self(Cow::Owned(byte))
    }

    /// Interpret a referenced byte as a type of device
    ///
    /// Does not check for correctness
    pub const fn new_borrowed_unchecked(byte: &'a u8) -> Self {
        Self(Cow::Borrowed(byte))
    }

    /// Get a reference to the underlying byte
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl fmt::Display for Controller<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
