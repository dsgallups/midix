use crate::prelude::*;
use core::fmt;

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
///
/// TODO
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Velocity(u8);

impl Velocity {
    /// Create a new velocity. Does not check for u7 correctness
    pub fn new(velocity: u8) -> Self {
        Self(velocity)
    }
}

impl MidiBits for Velocity {
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

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
///
/// TODO
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct VelocityRef<'a>(&'a u8);

impl<'a> VelocityRef<'a> {
    /// Create a new velocity
    pub(crate) const fn new(velocity: &'a u8) -> Self {
        Self(velocity)
    }
    pub const fn byte(&self) -> &u8 {
        self.0
    }
}
