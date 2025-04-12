use crate::prelude::*;
use core::fmt;

/// Identifies a device
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Controller(DataByte);

impl Controller {
    /// Interpret a byte as a type of device
    ///
    /// Checks for correctness (leading 0 bit)
    pub fn new<B, E>(rep: B) -> Result<Self, std::io::Error>
    where
        B: TryInto<DataByte, Error = E>,
        E: Into<io::Error>,
    {
        rep.try_into().map(Self).map_err(Into::into)
    }

    /// Get a reference to the underlying byte
    pub fn byte(&self) -> u8 {
        self.0 .0
    }
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
