use crate::prelude::*;
use core::fmt;

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Velocity(DataByte);

impl Velocity {
    /// Creates a new velocity from the provided byte
    ///
    /// Checks for correctness (leading 0 bit)
    pub fn new<B, E>(rep: B) -> Result<Self, std::io::Error>
    where
        B: TryInto<DataByte, Error = E>,
        E: Into<io::Error>,
    {
        rep.try_into().map(Self).map_err(Into::into)
    }

    /// Returns a max velocity
    pub fn max() -> Self {
        Self(DataByte::new_unchecked(127))
    }

    /// Returns a velocity of zero.
    pub fn zero() -> Self {
        Self(DataByte::new_unchecked(0))
    }

    /// Get a reference to the underlying byte
    pub fn byte(&self) -> u8 {
        self.0.0
    }

    /// Get the dynamic of the velocity...fortississississimo
    pub fn dynamic(&self) -> Dynamic {
        match self.byte() {
            0 => Dynamic::off(),
            1..16 => Dynamic::ppp(),
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

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The musical analog of the digital velocity
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dynamic {
    /// no sound
    Off,
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
    /// No sound
    pub fn off() -> Self {
        Self::Off
    }

    /// very quiet
    pub fn ppp() -> Self {
        Self::Pianississimo
    }

    /// pretty quiet
    pub fn pp() -> Self {
        Self::Pianissimo
    }

    /// quiet
    pub fn p() -> Self {
        Self::Piano
    }

    /// kinda quiet
    pub fn mp() -> Self {
        Self::MezzoPiano
    }

    /// kinda loud
    pub fn mf() -> Self {
        Self::MezzoForte
    }

    /// loud
    pub fn f() -> Self {
        Self::Forte
    }

    /// pretty loud
    pub fn ff() -> Self {
        Self::Fortissimo
    }

    /// very loud
    pub fn fff() -> Self {
        Self::Fortississimo
    }
}
