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
}
