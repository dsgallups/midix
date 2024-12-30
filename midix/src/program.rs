use crate::prelude::*;
use core::fmt;

/// Identifies an instrument
///
/// TODO docs
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Program<'a>(DataByte<'a>);

impl<'a> Program<'a> {
    /// Creates a new program command.
    ///
    /// Does not check for correctness.
    pub fn new<B, E>(rep: B) -> Result<Self, std::io::Error>
    where
        B: TryInto<DataByte<'a>, Error = E>,
        E: Into<io::Error>,
    {
        rep.try_into().map(Self).map_err(Into::into)
    }

    /// Get a reference to the underlying byte for the program.
    pub fn byte(&self) -> &u8 {
        self.0.byte()
    }
}

impl fmt::Display for Program<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
