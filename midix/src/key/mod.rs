use core::fmt;

use crate::num::u7;

/// Identifies a key press
///
/// TODO docs
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Key(u7);

impl Key {
    /// Create a new key
    pub fn new(key: impl Into<u7>) -> Self {
        Self(key.into())
    }

    /// Writes out as int
    pub fn as_int(self) -> u8 {
        self.0.as_int()
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
