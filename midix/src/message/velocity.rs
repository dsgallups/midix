use crate::prelude::*;
use core::fmt;

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
///
/// TODO
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Velocity(u8);

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

impl Velocity {
    /// Create a new velocity
    pub fn new(velocity: u8) -> Self {
        Self(velocity)
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
