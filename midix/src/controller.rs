use crate::prelude::*;
use core::fmt;

/// Identifies a device
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Controller<'a>(DataByte<'a>);

impl<'a> Controller<'a> {
    /// Interpret a byte as a type of device
    ///
    /// Checks for correctness (leading 0 bit)
    pub fn new<B, E>(rep: B) -> Result<Self, std::io::Error>
    where
        B: TryInto<DataByte<'a>, Error = E>,
        E: Into<io::Error>,
    {
        rep.try_into().map(Self).map_err(Into::into)
    }

    /// Get a reference to the underlying byte
    pub fn byte(&self) -> &u8 {
        self.0.byte()
    }
}

impl fmt::Display for Controller<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
