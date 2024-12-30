use crate::prelude::*;
use core::fmt;
use std::borrow::Cow;

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Velocity<'a>(Cow<'a, u8>);

impl<'a> Velocity<'a> {
    /// Creates a new velocity from the provided byte
    ///
    /// Does not check for correctness
    pub const fn new_unchecked(velocity: u8) -> Self {
        Self(Cow::Owned(velocity))
    }
    /// Create a new velocity from the referenced byte
    ///
    /// Does not check for correctness
    pub const fn new_borrowed_unchecked(velocity: &'a u8) -> Self {
        Self(Cow::Borrowed(velocity))
    }
    /// Creates a new velocity from the provided byte
    ///
    /// Checks for correctness (leading 0 bit)
    pub fn new(rep: u8) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Self(Cow::Owned(check_u7(rep)?)))
    }

    /// Get a reference to the underlying byte
    pub fn byte(&self) -> &u8 {
        &self.0
    }

    /// Get a reference to the underlying byte
    pub fn value(&self) -> u8 {
        *self.0
    }

    /// Get the dynamic of the velocity...fortississississimo
    pub fn dynamic(&self) -> Dynamic {
        match self.value() {
            0..16 => Dynamic::ppp(),
            16..32 => Dynamic::pp(),
            32..48 => Dynamic::p(),
            48..64 => Dynamic::mp(),
            64..80 => Dynamic::mf(),
            80..96 => Dynamic::f(),
            96..112 => Dynamic::ff(),
            _ => Dynamic::fff(),
        }
    }
}

impl fmt::Display for Velocity<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The musical analog of the digital velocity
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dynamic {
    /// very quiet
    Pianississimo,
    /// pretty quiet
    Pianissimo,
    /// quiet
    Piano,
    /// kinda quiet
    MezzoPiano,
    /// kinda loud
    MezzoForte,
    /// loud
    Forte,
    /// pretty loud
    Fortissimo,
    /// very loud
    Fortississimo,
}

impl Dynamic {
    fn ppp() -> Self {
        Self::Pianississimo
    }
    fn pp() -> Self {
        Self::Pianissimo
    }
    fn p() -> Self {
        Self::Piano
    }
    fn mp() -> Self {
        Self::MezzoPiano
    }
    fn mf() -> Self {
        Self::MezzoForte
    }
    fn f() -> Self {
        Self::Forte
    }
    fn ff() -> Self {
        Self::Fortissimo
    }
    fn fff() -> Self {
        Self::Fortississimo
    }
}
