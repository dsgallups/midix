use crate::prelude::*;
use core::fmt;

/// Identifies a controller
///
/// TODO docs
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Controller(u8);

impl MidiBits for Controller {
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

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
