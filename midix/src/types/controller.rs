use crate::prelude::*;
use core::fmt;
use std::borrow::Cow;

/// Identifies a controller
///
/// TODO docs
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Controller<'a>(Cow<'a, u8>);

impl<'a> Controller<'a> {
    pub(crate) const fn new(byte: u8) -> Self {
        Self(Cow::Owned(byte))
    }
    pub(crate) const fn new_borrowed(byte: &'a u8) -> Self {
        Self(Cow::Borrowed(byte))
    }
}

impl MidiBits for Controller<'_> {
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

impl fmt::Display for Controller<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
