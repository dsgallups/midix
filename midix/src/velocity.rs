use core::fmt;

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
///
/// TODO
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Velocity(u8);

impl Velocity {
    /// Create a new velocity
    pub fn new(velocity: impl Into<u8>) -> Self {
        Self(velocity.into())
    }
    /// As an int
    pub fn as_int(self) -> u8 {
        self.0
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
